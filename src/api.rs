//  Created by 有个小小杜

use crate::assets::DocsFolder;
use dioxus::prelude::*;

pub fn docs_content(section: String, item: String) -> Option<String> {
    let docs_content = DocsFolder::get(&format!("{}/{}.md", section, item));
    dioxus::logger::tracing::debug!("docs_content: {}", &format!("{}/{}.md", section, item));
    if let Some(docs) = docs_content {
        Some(std::str::from_utf8(&docs.data).unwrap().to_string())
    } else {
        dioxus::logger::tracing::debug!("docs_content: {} not find", &format!("{}/{}.md", section, item));
        None
    }
}

#[post("/api/docs/content")]
pub async fn post_docs_content(section: String, item: String) -> Result<String, ServerFnError> {
    if let Some(docs) = docs_content(section, item) {
        Ok(docs)
    } else {
        Err(ServerFnError::Response("Not Found".to_string()))
    }
}
