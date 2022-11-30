//! Serving static files.

use axum::{
    body::{boxed, Full},
    extract::State,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;
use sailfish::TemplateOnce;

use crate::objects::OMConfig;

#[derive(RustEmbed)]
#[folder = "./dist"]
#[exclude = "index.js"]
struct Asset;

#[derive(TemplateOnce)]
#[template(path = "index.js", escape = false, delimiter = '@')]
struct JsReplace<'a> {
    base_url: &'a str,
    ws_url: &'a str,
}

/// Return the index.
pub async fn index() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap_or_default()).await
}

/// Return the static file.
pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();

    StaticFile(path)
}

/// Return the index.js file, replacing the base URL and websocket URL.
pub async fn index_js(State(config): State<OMConfig>) -> impl IntoResponse {
    let content = JsReplace {
        base_url: config.base_url.as_str(),
        ws_url: config.ws_url.as_str(),
    }
    .render_once()
    .unwrap();

    let body = boxed(Full::from(content));
    let mime = mime_guess::from_path("index.js").first_or_octet_stream();
    Response::builder()
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(body)
        .unwrap()
}

/// A static file.
pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        if let Some(content) = Asset::get(path.as_str()) {
            let body = boxed(Full::from(content.data));
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body)
                .unwrap()
        } else {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("404")))
                .unwrap()
        }
    }
}
