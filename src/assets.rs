use dioxus::prelude::*;
use rust_embed::RustEmbed;

// server
#[derive(RustEmbed)]
#[folder = "docs/"]
pub struct DocsFolder;

// client
pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const THEME_CSS: Asset = asset!("/assets/theme.css");
pub const STYLES_CSS: Asset = asset!("/assets/styles.css");
pub const ONLOAD_JS: Asset = asset!("/assets/onload.js");
pub const LOGO: Asset = asset!(
    "/assets/logo.png",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Avif)
);
