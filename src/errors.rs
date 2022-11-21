use rocket::{
    http::{ContentType, Status},
    response::Responder,
    Response,
};
use std::io::Cursor;
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
    InvalidName,
    #[error("Invalid password.")]
    InvalidPassword,
    #[error("Failed to make network request.")]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error("Failed to hash password.")]
    ArgonError,
}

impl OMError {
    pub fn to_status(&self) -> Status {
        use OMError::*;

        match self {
            SqlxError(_) | ReqwestError(_) | ArgonError => Status::InternalServerError,
            InvalidSession | InvalidPassword => Status::Unauthorized,
            NotFound(_) => Status::NotFound,
            NameTaken => Status::Conflict,
            NoPermission => Status::Forbidden,
            InvalidName => Status::BadRequest,
        }
    }
}

impl<'r> Responder<'r, 'static> for OMError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let body = self.to_string();

        let resp = Response::build()
            .header(ContentType::Plain)
            .status(self.to_status())
            .sized_body(body.len(), Cursor::new(body))
            .finalize();
        Ok(resp)
    }
}
