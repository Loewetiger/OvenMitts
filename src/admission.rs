//! Handle the logic that determines whether a stream is allow or denied.

use rocket_db_pools::Connection;

use crate::{
    db::Mitts,
    objects::{Admission, AdmissionResponse, User},
};

/// Check whether the incoming stream is allowed to stream.
///
/// This function does the following:
/// - Retrieve the url sent by OvenMediaEngine and split the path into a [Vec]
/// - Pop off the last element, which *should* be the stream key
/// - Query the database for said key
/// - If a user is found: allow the stream and rewrite the url to point to their username
/// - If no user is found, or a database error occurs, deny the stream
pub async fn handle_admission(adm: Admission, mut db: Connection<Mitts>) -> AdmissionResponse {
    let mut url = adm.borrow_url().clone();
    let mut path = match url.path_segments().map(|c| c.collect::<Vec<_>>()) {
        Some(vec) => vec,
        None => return AdmissionResponse::deny(),
    };
    // Get the last element, which should be the stream key
    let stream_key = path.pop().unwrap_or("");
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE stream_key = ?", stream_key)
        .fetch_one(&mut *db)
        .await;

    match user {
        Ok(user) => {
            path.push(&user.username);
            url.set_path(&path.join("/"));
            AdmissionResponse::allow(url)
        }
        Err(_) => AdmissionResponse::deny(),
    }
}
