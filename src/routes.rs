//! The routes for the webserver.

use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::{db::Mitts, objects::{AdmissionResponse, Admission}, admission::handle_admission};

#[post("/admission", data = "<adm>")]
pub fn post_admission(adm: Json<Admission>, db: Connection<Mitts>) -> Json<AdmissionResponse> {
    Json(handle_admission(adm.into_inner(), db))
}
