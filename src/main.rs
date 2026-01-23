use dioxus::prelude::*;
use ktm_site::app::App;

fn main() {
    dioxus::launch(App);
}

/// Echo the user input on the server.
#[post("/api/echo")]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
