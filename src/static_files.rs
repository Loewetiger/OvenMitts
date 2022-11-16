//! Logic for embedding and serving the static files, to be able to have a single binary.

use std::{borrow::Cow, ffi::OsStr, path::PathBuf};

use rocket::{http::ContentType, response::content::RawHtml, State};
use rust_embed::RustEmbed;
use sailfish::TemplateOnce;

use crate::objects::Config;

#[derive(RustEmbed)]
#[folder = "./dist"]
#[exclude = "index.js"]
struct Asset;

#[derive(TemplateOnce)]
#[template(path = "index.js", escape = false, delimiter = '@')]
struct JsReplace {
    base_url: String,
    ws_url: String,
}

/// Serves the `index.html` file.
#[get("/")]
pub fn get_index() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = Asset::get("index.html")?;
    Some(RawHtml(asset.data))
}

/// Serves the `index.js` file, with the `base_url` and `ws_url` replaced.
#[get("/index.js")]
pub fn get_indexjs(config: &State<Config>) -> Option<(ContentType, String)> {
    let ctx = JsReplace {
        base_url: config.base_url.clone().into(),
        ws_url: config.ws_url.clone().into(),
    };
    let js = ctx.render_once().ok()?;
    Some((ContentType::JavaScript, js))
}

/// Serves the static files in the `assets` directory.
#[get("/assets/<file..>")]
pub fn get_assets(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = format!("assets/{}", file.display());
    let asset = Asset::get(&filename)?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);

    Some((content_type, asset.data))
}
