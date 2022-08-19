//! The routes for the webserver.

/// Quick hello world route to test the basic syntax.
/// 
/// Just always returns a `hello world` string.
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
