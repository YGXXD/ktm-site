use crate::assets::*;
use crate::page::docs::Docs;
use crate::components::nav_bar::*;
use dioxus::prelude::*;

#[component]
fn Home() -> Element {
    rsx! {
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navagation)]
    #[route("/")]
    Home {},
    #[route("/docs")]
    Docs {},
}

#[component]
fn Navagation() -> Element {
    rsx! {
        div {
            class: "nav-bar-area",
            div {
                class: "nav-bar-content",
                NavBar {
                    logo: Some(NavBarLogoProps {
                        src: LOGO,
                        height: 50,
                    }),
                    items: vec![
                        NavBarItemProps {
                            label: "Home".to_owned(),
                            to: NavigationTarget::Internal(Route::Home {}).into(),
                        },
                        NavBarItemProps {
                            label: "Docs".to_owned(),
                            to: NavigationTarget::Internal(Route::Docs {}).into(),
                        },
                        NavBarItemProps {
                            label: "Github".to_owned(),
                            to: NavigationTarget::External("https://github.com/YGXXD/ktm".to_string())
                        }
                    ]
                }
            }
        }
        div {
            class: "page-area",
            Outlet::<Route> { }
        }
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Title { "ktm - 属于珂学家们的数学库" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: THEME_CSS }
        document::Link { rel: "stylesheet", href: STYLES_CSS }
        script { src: ONLOAD_JS }
        Router::<Route> {}
    }
}
