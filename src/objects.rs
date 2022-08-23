//! All the various structs for JSON/db support.

use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    Request,
};
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{db::Mitts, queries::get_user_by_session};

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
    /// V4 UUID.
    pub id: String,
    /// Username, will be used for URL rewrite.
    pub username: String,
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
            .get("user_session")
            .and_then(|c| c.value().parse().ok());

        match cookie {
            Some(token) => {
                let user = get_user_by_session(&token, &mut *db).await;
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
    /// V4 UUID.
    pub id: String,
    /// Username, used for displaying.
    pub username: String,
    /// Stream key, used for displaying.
    pub stream_key: String,
}

impl From<User> for SendableUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            stream_key: user.stream_key,
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
