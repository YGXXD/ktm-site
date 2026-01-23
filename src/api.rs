use crate::assets::DocsFolder;
use dioxus::prelude::*;

#[post("/api/docs/menulist")]
pub async fn docs_menulist() -> Result<String, ServerFnError> {
    let config = DocsFolder::get("config.json");
    if let Some(config) = config {
        Ok(std::str::from_utf8(&config.data).unwrap().to_string())
    } else {
        Err(ServerFnError::Response("Not Found".to_string()))
    }
}

#[post("/api/docs/content")]
pub async fn docs_content(section: String, item: String) -> Result<String, ServerFnError> {
    let docs_content = DocsFolder::get(&format!("{}/{}.md", section, item));
    if let Some(docs) = docs_content {
        Ok(std::str::from_utf8(&docs.data).unwrap().to_string())
    } else {
        Err(ServerFnError::Response("Not Found".to_string()))
    }
}
