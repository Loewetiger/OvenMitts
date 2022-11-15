//! A set of tools built around `OvenMediaEngine`, which makes hosting your own low-latency streaming server easy.

#![warn(missing_docs)]
#[macro_use]
extern crate rocket;

mod admission;
pub mod crypto;
pub mod db;
pub mod objects;
pub mod routes;
pub mod static_files;
