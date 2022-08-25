//! Authentication and authorization logic.

use rocket::{outcome::Outcome, request::FromRequest};

use crate::objects::User;

/// Read the permissions of the currently logged in user from the database.
pub async fn extract_permissions(req: &rocket::Request<'_>) -> Option<Vec<String>> {
    let user = match User::from_request(req).await {
        Outcome::Success(g) => g,
        _ => return None,
    };
    user.permission_vec()
}
