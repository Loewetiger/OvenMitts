//! Handle the logic that determines whether a stream is allow or denied.

use rocket_db_pools::Connection;

use crate::{
    db::Mitts,
    objects::{Admission, AdmissionResponse, User},
};

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
