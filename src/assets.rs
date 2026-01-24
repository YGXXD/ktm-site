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
pub const DOCS_CONFIG: Asset = asset!("/assets/docs_config.json");
// let mock_sections: Vec<MenuListSectionProps> = vec![
//     MenuListSectionProps {
//         title: "section1".to_owned(),
//         items: vec![
//             MenuListItemProps {
//                 label: "add".to_owned(),
//                 to: NavigationTarget::Internal(SiteRoute::DocsContent {
//                     section: "section1".to_owned(),
//                     item: "add".to_owned(),
//                 })
//                 .into(),
//             },
//             MenuListItemProps {
//                 label: "sub".to_owned(),
//                 to: NavigationTarget::Internal(SiteRoute::DocsContent {
//                     section: "section1".to_owned(),
//                     item: "sub".to_owned(),
//                 })
//                 .into(),
//             },
//         ],
//     },
//     MenuListSectionProps {
//         title: "section2".to_owned(),
//         items: vec![
//             MenuListItemProps {
//                 label: "mul".to_owned(),
//                 to: NavigationTarget::Internal(SiteRoute::DocsContent {
//                     section: "section2".to_owned(),
//                     item: "mul".to_owned(),
//                 })
//                 .into(),
//             },
//             MenuListItemProps {
//                 label: "div".to_owned(),
//                 to: NavigationTarget::Internal(SiteRoute::DocsContent {
//                     section: "section2".to_owned(),
//                     item: "div".to_owned(),
//                 })
//                 .into(),
//             },
//         ],
//     },
// ];