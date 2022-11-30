//! A set of tools built around `OvenMediaEngine`, which makes hosting your own low-latency streaming server easy.

#![warn(missing_docs)]

use lazy_static::lazy_static;
use regex::Regex;
use sqlx::{Pool, Sqlite};

mod crypto;
mod errors;
pub mod objects;
pub mod routes;
pub mod static_files;

/// The database connection pool.
pub type Db = Pool<Sqlite>;

lazy_static! {
    /// A regex that matches a valid username.
    ///
    /// A valid username must:
    /// - be between 4 and 25 characters long
    /// - only contain alphanumeric characters and underscores
    pub static ref USERNAME_RE: Regex = Regex::new("^[a-zA-Z0-9_]{4,25}$").unwrap();
}
