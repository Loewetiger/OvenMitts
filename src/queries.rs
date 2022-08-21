//! Common SQL queries for OvenMitts.

use rocket_db_pools::Connection;

use crate::{db::Mitts, objects::User};

pub async fn get_user_by_session(session: &str, mut db: Connection<Mitts>) -> Option<User> {
    sqlx::query_as!(
        User,
        "
        SELECT users.* FROM users
        LEFT JOIN sessions
        ON users.id = sessions.user_id
        WHERE session = ?
        ",
        session
    )
    .fetch_one(&mut *db)
    .await
    .ok()
}
