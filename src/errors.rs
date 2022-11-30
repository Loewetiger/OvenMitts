use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OMError {
    #[error("Invalid session.")]
    InvalidSession,
    #[error("Username `{0}` not found.")]
    NotFound(String),
    #[error("Username is already taken.")]
    NameTaken,
    #[error("You don't have permission to do that.")]
    NoPermission,
    #[error("Username contains invalid characters.")]
    InvalidUsername,
    #[error("Invalid password.")]
    InvalidPassword,
    #[error(transparent)]
    ReqwestError(reqwest::Error),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error("{0}")]
    ArgonError(String),
}

impl From<reqwest::Error> for OMError {
    fn from(e: reqwest::Error) -> Self {
        OMError::ReqwestError(e.without_url())
    }
}

impl From<argon2::password_hash::Error> for OMError {
    fn from(e: argon2::password_hash::Error) -> Self {
        match e {
            argon2::password_hash::Error::Password => OMError::InvalidPassword,
            _ => OMError::ArgonError(e.to_string()),
        }
    }
}

impl OMError {
    const fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidSession => StatusCode::UNAUTHORIZED,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::NameTaken => StatusCode::CONFLICT,
            Self::NoPermission | Self::InvalidPassword => StatusCode::FORBIDDEN,
            Self::InvalidUsername => StatusCode::BAD_REQUEST,
            Self::SqlxError(_)
            | Self::JoinError(_)
            | Self::ArgonError(_)
            | Self::ReqwestError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for OMError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code(), self.to_string()).into_response()
    }
}
