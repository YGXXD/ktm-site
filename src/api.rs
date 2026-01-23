use crate::assets::DocsFolder;
use dioxus::prelude::*;

#[post("/api/docs/menulist")]
pub async fn docs_menulist() -> Result<String, ServerFnError> {
    let config = DocsFolder::get("config.json");
    let result = if let Some(config) = config {
        std::str::from_utf8(&config.data).unwrap().to_string()
    } else {
        "".to_string()
    };
    Ok(result)
}

#[post("/api/docs/content")]
pub async fn docs_dontent(section: String, item: String) -> Result<String, ServerFnError> {
    // todo
    Ok(section)
}
