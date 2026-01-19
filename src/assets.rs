use dioxus::prelude::*;

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const THEME_CSS: Asset = asset!("/assets/theme.css");
pub const STYLES_CSS: Asset = asset!("/assets/styles.css");
pub const ONLOAD_JS: Asset = asset!("/assets/onload.js");
pub const LOGO: Asset = asset!("/assets/logo.png");
pub const MD: &str = include_str!("../README.md");