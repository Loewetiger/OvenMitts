//! All the various structs for JSON/db support.

use std::{borrow::Cow, str::FromStr};

use chrono::{DateTime, FixedOffset};
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    response::Responder,
    Request, Response,
};
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::db::{Db, Mitts};

/// Response to `OvenMediaEngine`'s admission webhook.
#[derive(Debug, Serialize)]
pub struct AdmissionResponse {
    allowed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_url: Option<Url>,
}

impl AdmissionResponse {
    /// Returns an [`AdmissionResponse`] that allows streaming under a new url (most common, because a valid stream key has been supplied).
    /// The new url should not contain the stream key.
    ///
    /// The resulting JSON will look like this:
    /// ```json
    /// {
    ///   allowed: true,
    ///   new_url: "https://example.com/stream/Username"
    /// }
    /// ```
    #[must_use]
    pub const fn allow(new_url: Url) -> Self {
        Self {
            allowed: true,
            new_url: Some(new_url),
        }
    }
    /// Returns an [`AdmissionResponse`] that denys streaming (because an invalid stream key has been supplied, or an internal error occured).
    ///
    /// The resulting JSON will look like this:
    /// ```json
    /// {
    ///   allowed: false
    /// }
    /// ```
    #[must_use]
    pub const fn deny() -> Self {
        Self {
            allowed: false,
            new_url: None,
        }
    }
}

/// The request that is sent by `OvenMediaEngine`.
///
/// Most of the request gets discarded, since all that's really needed for `OvenMitts` is the `url` field, because the stream key is found there.
/// ```json
/// {
///   "request": {
///     "url": "rtmp://example.com/stream/secret_stream_key"
///   }
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct Admission {
    request: AdmissionRequest,
}

/// Helper struct to retrive the nested url.
#[derive(Debug, Deserialize)]
struct AdmissionRequest {
    url: Url,
}

impl Admission {
    /// Returns the url which contains the stream key in it's path.
    #[must_use]
    pub const fn borrow_url(&self) -> &Url {
        &self.request.url
    }
}

/// The representation of a user in the database.
#[derive(Debug)]
pub struct User {
    /// Username, will be used for URL rewrite.
    pub username: String,
    /// The name that gets displayed in the UI.
    pub display_name: String,
    /// Argon2id hased password.
    pub password: String,
    /// Randomly generated stream key.
    pub stream_key: String,
    /// The various grants that the user has.
    pub permissions: Option<String>,
    /// Title of the stream.
    pub stream_title: Option<String>,
}

impl User {
    /// Returns a Vec of all the permissions.
    pub fn permission_vec(&self) -> Option<Vec<String>> {
        Some(
            self.permissions
                .as_ref()?
                .split(',')
                .map(std::string::ToString::to_string)
                .collect(),
        )
    }
    /// Check whether the user has a specified permission
    pub fn has_permission(&self, permission: String) -> bool {
        let perms = self.permission_vec();
        match perms {
            Some(v) => v.contains(&permission),
            None => false,
        }
    }
    /// Find a User from the database from the username.
    pub async fn from_name(username: &str, db: &mut Db) -> Option<Self> {
        sqlx::query_as!(
            User,
            "
        SELECT * FROM users
        WHERE users.username = ? COLLATE NOCASE
        ",
            username
        )
        .fetch_one(db)
        .await
        .ok()
    }
    /// Check if a given username can be found in the database.
    /// This is case-insensitive.
    pub async fn username_exists(username: &str, db: &mut Db) -> bool {
        Self::from_name(username, db).await.is_some()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let mut db: Connection<Mitts> = match Connection::from_request(req).await {
            Outcome::Success(db) => db,
            Outcome::Failure(_) => {
                return Outcome::Failure((
                    Status::InternalServerError,
                    "Failed to establish database connection",
                ))
            }
            Outcome::Forward(f) => return Outcome::Forward(f),
        };

        let cookie: Option<String> = req
            .cookies()
            .get_private("user_session")
            .and_then(|c| c.value().parse().ok());

        match cookie {
            Some(token) => {
                let session = SessionCookie::from_str(&token).unwrap_or_default();
                if !session.is_valid() {
                    return Outcome::Failure((Status::Unauthorized, "Invalid session"));
                }

                let user = session.get_user(&mut db).await;

                match user {
                    Some(user) => Outcome::Success(user),
                    None => Outcome::Failure((Status::Unauthorized, "No user found in database")),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, "No session")),
        }
    }
}

#[derive(Serialize, Debug)]
/// Like the [User] struct, but without the hashed password. Inteded to be sent to the frontend.
pub struct SendableUser {
    /// URL-safe Username, used for streaming.
    pub username: String,
    /// Name that gets displayed in the UI.
    pub display_name: String,
    /// Stream key, used for displaying.
    pub stream_key: String,
    /// Optional stream title.
    pub stream_title: Option<String>,
}

impl From<User> for SendableUser {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
            display_name: user.display_name,
            stream_key: user.stream_key,
            stream_title: user.stream_title,
        }
    }
}

#[derive(Debug, Deserialize)]
/// The request that is sent by the frontend when a user wants to login.
pub struct LoginUser {
    /// Username of the user.
    pub username: String,
    /// Password of the user.
    pub password: String,
}

/// List of all current streams from OvenMediaEngine.
#[derive(Debug, Deserialize)]
pub struct Streams {
    /// The vec containing all the stream names.
    pub response: Vec<String>,
}

#[derive(Debug, Serialize)]
/// Response for stream info.
pub struct StreamResp {
    /// Username of the streaming user, URL safe.
    pub username: String,
    /// Name the gets displayed in the UI.
    pub display_name: String,
    /// Optional stream title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Custom configuration for the OvenMitts.
#[derive(Debug, serde::Deserialize)]
pub struct Config {
    /// The url base to access OvenMediaEngine.
    pub ome_url: Url,
    /// OME API access token.
    pub access_token: String,
    /// The key used by OME to sign the admission requests.
    pub admission_key: String,
}

/// Wrapper type for reqwest to simplify error handling within rocket.
pub struct ReqwestError(reqwest::Error);

impl<'r> Responder<'r, 'r> for ReqwestError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'r> {
        // Censor the url in the error message, since it might contain sensitive information.
        let err_msg = self.0.without_url().to_string();

        Ok(Response::build()
            .sized_body(err_msg.len(), std::io::Cursor::new(err_msg))
            .status(Status::InternalServerError)
            .finalize())
    }
}

impl From<reqwest::Error> for ReqwestError {
    fn from(e: reqwest::Error) -> Self {
        Self(e)
    }
}

/// Struct to fill the encrypted session cookies.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SessionCookie {
    /// The corresponding username.
    u: String,
    /// The time until the cookie expires.
    t: DateTime<FixedOffset>,
}

impl SessionCookie {
    /// Create a new session cookie.
    pub fn new(username: String, valid_until: DateTime<FixedOffset>) -> Self {
        Self {
            u: username,
            t: valid_until,
        }
    }
    /// Check if the session cookie is still valid.
    pub fn is_valid(&self) -> bool {
        self.t >= chrono::offset::Utc::now()
    }
    /// Get the user from the database.
    pub async fn get_user(&self, db: &mut Db) -> Option<User> {
        User::from_name(&self.u, db).await
    }
}

impl<'c> From<SessionCookie> for Cow<'c, str> {
    fn from(val: SessionCookie) -> Self {
        let cookie = serde_json::to_string(&val).unwrap_or_default();
        Cow::from(cookie)
    }
}

impl FromStr for SessionCookie {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
