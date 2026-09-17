//  Created by 有个小小杜

use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};

#[component]
pub fn MarkDown(content: Option<String>) -> Element {
    if let Some(content) = content {
        let parser = Parser::new_ext(content.as_str(), Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        rsx! {
            div {
                class: "markdown-body",
                dangerous_inner_html: html_output
            }
        }
    } else {
        rsx! {}
    }
}
