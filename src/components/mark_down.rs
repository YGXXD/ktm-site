use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};

#[component]
pub fn MarkDown(content: Option<String>) -> Element {
    let inner_html = if let Some(content) = content {
        let parser = Parser::new_ext(content.as_str(), Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    } else {
        "".to_string()
    };

    rsx! {
        div {
            dangerous_inner_html: inner_html
        }
    }
}
