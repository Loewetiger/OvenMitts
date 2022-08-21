//! The routes for the webserver.

use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::{
    admission::handle_admission,
    auth::AuthGuard,
    db::Mitts,
    objects::{Admission, AdmissionResponse, SendableUser},
};

/// Used by OvenMediaEngine's [admission webhooks](https://airensoft.gitbook.io/ovenmediaengine/access-control/admission-webhooks).
///
/// Checks if the given stream key is in the database and rewrites the stream url to point to the users' username.
/// It does **not** currently check the headers for the HMAC signature provided by OME.
#[post("/admission", data = "<adm>")]
pub async fn post_admission(
    adm: Json<Admission>,
    db: Connection<Mitts>,
) -> Json<AdmissionResponse> {
    Json(handle_admission(adm.into_inner(), db).await)
}

#[get("/user")]
pub fn get_user(guard: AuthGuard) -> Json<SendableUser> {
    Json(guard.user.into())
}
