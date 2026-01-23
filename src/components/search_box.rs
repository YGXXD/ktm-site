use dioxus::prelude::*;

#[component]
pub fn SearchBox(placeholder: String, oninput: EventHandler<String>) -> Element {
    let callback = oninput;
    let oninput = move |event: Event<FormData>| {
        let value = event.value();
        callback(value);
    };

    rsx! {
        div {
            input {
                style: r#"
                    width: 100%;
                    padding: 0.5rem 0.75rem;
                    border-width: 3px;
                    border-radius: 12px;
                "#,
                r#type: "text",
                placeholder: placeholder,
                oninput: oninput
            }
        }
    }
}
