//! Various structs for JSON objects and database models.

use axum::extract::{FromRef, State};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf};
use tower_cookies::Cookies;
use url::Url;

use crate::{errors::OMError, Db};

/// Session data for a user.
#[derive(Debug, Deserialize)]
pub struct Session {
    /// Randomly generated session-id.
    pub session: String,
    /// The associated user.
    pub user_id: String,
    /// Time of creation in UTC.
    pub created_at: NaiveDateTime,
}

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
#[derive(Debug, Clone)]
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
    /// Check whether the user has a specified permission
    #[must_use]
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.as_ref().map_or(false, |permissions| {
            permissions.contains(permission) || permissions.contains("IS_ADMIN")
        })
    }
    /// Check whether the user is an admin
    #[must_use]
    pub fn is_admin(&self) -> bool {
        self.has_permission("IS_AMDIN")
    }
    /// Find a User from the database from the username.
    /// Case-insensitive.
    pub async fn from_name(username: &str, db: &Db) -> Option<Self> {
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
    /// Find a User in the database for a given token.
    pub async fn from_session(token: &str, db: &Db) -> Option<Self> {
        sqlx::query_as!(
            User,
            "
        SELECT users.* FROM users
        LEFT JOIN sessions
        ON users.username = sessions.user_id
        WHERE session = ?
        ",
            token
        )
        .fetch_one(db)
        .await
        .ok()
    }
    /// Find a User in the database for a given cookie.
    pub async fn from_req(State(db): State<Db>, cookies: Cookies) -> Result<Self, OMError> {
        let om_cookie = cookies.get("om_session").ok_or(OMError::InvalidSession)?;
        let session = om_cookie.value();
        User::from_session(session, &db)
            .await
            .map(|u| u)
            .ok_or(OMError::InvalidSession)
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_title: Option<String>,
    /// The current permissions of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
}

impl From<User> for SendableUser {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
            display_name: user.display_name,
            stream_key: user.stream_key,
            stream_title: user.stream_title,
            permissions: user.permissions,
        }
    }
}

/// Payload for logging in and registering.
#[derive(Debug, Deserialize)]
pub struct UserLogin {
    /// Username of the user.
    pub username: String,
    /// Password of the user.
    pub password: String,
}

/// Custom configuration for OvenMitts.
#[derive(Debug, Deserialize, Clone)]
pub struct OMConfig {
    #[serde(default = "default_address")]
    /// Address to bind to.
    pub address: SocketAddr,
    #[serde(default = "default_database")]
    /// Path to the database
    pub database: PathBuf,
    /// The url base to access OvenMediaEngine.
    pub ome_url: Url,
    /// OME API access token.
    pub access_token: String,
    /// The key used by OME to sign the admission requests.
    pub admission_key: String,
    /// The url base to access OvenMitts.
    pub base_url: Url,
    /// Websocket url for the player.
    pub ws_url: Url,
}

fn default_address() -> SocketAddr {
    SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
        8000,
    )
}

fn default_database() -> PathBuf {
    PathBuf::from("mitts.sqlite")
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

#[derive(Debug, Clone)]
/// The state for the server.
pub struct AppState {
    /// The database pool
    pub db: Db,
    /// The configuration for the server.
    pub config: OMConfig,
}

impl FromRef<AppState> for Db {
    fn from_ref(input: &AppState) -> Self {
        input.db.clone()
    }
}

impl FromRef<AppState> for OMConfig {
    fn from_ref(input: &AppState) -> Self {
        input.config.clone()
    }
}

/// The struct used to update user attributes.
#[derive(Debug, Deserialize)]
pub struct UserUpdate {
    /// The user to be updated. If None, the currently logged in user will be used.
    pub username: Option<String>,
    /// The new display name.
    pub display_name: Option<String>,
    /// The new password.
    pub new_password: Option<String>,
    /// Old password, required to change the password.
    pub old_password: Option<String>,
    /// The new stream title.
    pub stream_title: Option<String>,
    /// The permissions, can only be set by admins.
    pub permissions: Option<String>,
}
