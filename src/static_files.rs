use std::{borrow::Cow, ffi::OsStr, path::PathBuf};

use rocket::{http::ContentType, response::content::RawHtml};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./dist"]
struct Asset;

#[get("/")]
pub fn get_index() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = Asset::get("index.html")?;
    Some(RawHtml(asset.data))
}

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
