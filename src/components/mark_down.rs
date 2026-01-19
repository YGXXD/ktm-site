use leptos::prelude::*;
use pulldown_cmark::{Options, Parser, html};

#[component]
pub fn MarkDown(content: String) -> impl IntoView {
    let parser = Parser::new_ext(content.as_str(), Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    view! {
        <div inner_html={html_output} />
    }
}
