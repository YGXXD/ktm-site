use stylist::{style, yew::styled_component};
use wasm_bindgen::JsCast;
use web_sys::console;
use yew::Callback;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SearchBoxProps {
    pub placeholder: String,
    pub onchange: Callback<String>,
}

#[styled_component(SearchBox)]
pub fn search_box(props: &SearchBoxProps) -> Html {
    let callback = props.onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let input = target.unchecked_ref::<web_sys::HtmlInputElement>();
        console::log_2(&"search docs: ".into(), &input.value().into());
        callback.emit(input.value())
    });

    let search_box_style_sheet = style!(
        r#"
            input {
                width: 100%;
                box-sizing: border-box;
                padding: 0.5rem 0.75rem;
                border-width: 3px;
                border-radius: 12px;
            }
        "#
    )
    .unwrap();
    html! {
        <div class={search_box_style_sheet.get_class_name().to_string()}>
            <input
                type="text"
                placeholder={props.placeholder.clone()}
                onchange={onchange}
            />
        </div>
    }
}
