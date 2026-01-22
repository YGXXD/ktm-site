use dioxus::prelude::*;
use pulldown_cmark::{Options, Parser, html};

pub const MD: &str = include_str!("../../README.md");

#[component]
pub fn MarkDown(path: String) -> Element  {
    let parser = Parser::new_ext(MD, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    rsx! {
        div {
            dangerous_inner_html: html_output
        }
    }
}
