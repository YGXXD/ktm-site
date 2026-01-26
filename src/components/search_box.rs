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
            class: "search-box",
            input {
                r#type: "text",
                placeholder: placeholder,
                oninput: oninput
            }
        }
    }
}
