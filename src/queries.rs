//! Common SQL queries for OvenMitts.

use sqlx::{pool::PoolConnection, Sqlite};

use crate::objects::User;

/// Get a user by their session token.
pub async fn get_user_by_session(session: &str, db: &mut PoolConnection<Sqlite>) -> Option<User> {
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
    .fetch_one(db)
    .await
    .ok()
}

/// Get a user by their username.
pub async fn get_user_by_name(name: &str, db: &mut PoolConnection<Sqlite>) -> Option<User> {
    sqlx::query_as!(
        User,
        "
        SELECT * FROM users
        WHERE users.username = ?
        ",
        name
    )
    .fetch_one(db)
    .await
    .ok()
}
