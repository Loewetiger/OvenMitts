//! Handle the logic that determines whether a stream is allow or denied.

use rocket_db_pools::Connection;

use crate::{objects::{AdmissionResponse, Admission}, db::Mitts};

pub fn handle_admission(adm: Admission, mut db: Connection<Mitts>) -> AdmissionResponse {
    let url = adm.borrow_url();
    let segments = url.path_segments().map(|c| c.collect::<Vec<&str>>());
    println!("{}", segments.unwrap()[1]);
    AdmissionResponse::deny()
}
