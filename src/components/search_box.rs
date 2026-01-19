use leptos::prelude::*;
use stylist::style;
use web_sys::console;

#[component]
pub fn SearchBox<F>(placeholder: String, oninput: F) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let callback = oninput;
    let oninput = move |event| {
        let value = event_target_value(&event);
        console::log_2(&"search docs: ".into(), &value.clone().into());
        callback(value);
    };

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
    view! {
        <div class={search_box_style_sheet.get_class_name().to_string()}>
            <input
                type="text"
                placeholder={placeholder}
                on:input={oninput}
            />
        </div>
    }
}
