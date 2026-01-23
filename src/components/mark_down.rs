use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};

pub const MD: &str = include_str!("../../README.md");

#[component]
pub fn MarkDown(path: Option<String>) -> Element {
    let inner_html = if let Some(path) = path {
        path
    } else {
        let parser = Parser::new_ext(MD, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    };

    rsx! {
        div {
            dangerous_inner_html: inner_html
        }
    }
}
