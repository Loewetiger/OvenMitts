//! Authentication and authorization logic.

use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    Request,
};
use rocket_db_pools::Connection;

use crate::{db::Mitts, objects::User, queries::get_user_by_session};

#[derive(Debug)]
/// A guard to guarantee that a user is logged in.
pub struct AuthGuard {
    /// The user that is logged in.
    pub user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
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
                    Some(user) => Outcome::Success(Self { user }),
                    None => Outcome::Failure((Status::Unauthorized, "No user found in database")),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, "No session")),
        }
    }
}

/// Read the permissions of the currently logged in user from the database.
pub async fn extract_permissions(req: &rocket::Request<'_>) -> Option<Vec<String>> {
    let guard = match AuthGuard::from_request(req).await {
        Outcome::Success(g) => g,
        _ => return None,
    };
    Some(
        guard
            .user
            .permissions
            .split(',')
            .map(std::string::ToString::to_string)
            .collect(),
    )
}
