use dioxus::prelude::*;
use pulldown_cmark::{Options, Parser, html};

#[component]
pub fn MarkDown(content: String) -> Element  {
    let parser = Parser::new_ext(content.as_str(), Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    rsx! {
        div {
            dangerous_inner_html: html_output
        }
    }
}
