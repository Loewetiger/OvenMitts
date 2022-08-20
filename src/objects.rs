//! All the various structs for JSON/db support.

use serde::{Deserialize, Serialize};
use url::Url;

/// Response to OvenMediaEngine's admission webhook.
#[derive(Debug, Serialize)]
pub struct AdmissionResponse {
    allowed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_url: Option<Url>,
}

impl AdmissionResponse {
    /// Returns an [AdmissionResponse] that allows streaming under a new url (most common, because a valid stream key has been supplied).
    /// The new url should not contain the stream key.
    ///
    /// The resulting JSON will look like this:
    /// ```json
    /// {
    ///   allowed: true,
    ///   new_url: "https://example.com/stream/Username"
    /// }
    /// ```
    pub fn allow(new_url: Url) -> Self {
        AdmissionResponse {
            allowed: true,
            new_url: Some(new_url),
        }
    }
    /// Returns an [AdmissionResponse] that denys streaming (because an invalid stream key has been supplied, or an internal error occured).
    ///
    /// The resulting JSON will look like this:
    /// ```json
    /// {
    ///   allowed: false
    /// }
    /// ```
    pub fn deny() -> Self {
        AdmissionResponse {
            allowed: false,
            new_url: None,
        }
    }
}

/// The request that is sent by OvenMediaEngine.
///
/// Most of the request gets discarded, since all that's really needed for OvenMitts is the `url` field, because the stream key is found there.
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

#[derive(Debug, Deserialize)]
struct AdmissionRequest {
    url: Url,
}

impl Admission {
    /// Returns the url which contains the stream key in it's path.
    pub fn borrow_url(&self) -> &Url {
        &self.request.url
    }
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub stream_key: String,
    pub is_admin: bool,
    pub can_stream: bool,
    pub can_restream: bool,
    pub can_privatestream: bool,
}
