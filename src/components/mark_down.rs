use pulldown_cmark::{Options, Parser, html};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MarkDownProps {
    pub content: String,
}

#[function_component(MarkDown)]
pub fn mark_down(props: &MarkDownProps) -> Html {
    let parser = Parser::new_ext(props.content.as_str(), Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html! {
        <div>
            {Html::from_html_unchecked(html_output.into())}
        </div>
    }
}
