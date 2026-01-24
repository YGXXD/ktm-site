use crate::assets::*;
use crate::components::nav_bar::*;
use crate::page::docs::*;
use dioxus::prelude::*;

#[component]
fn Home() -> Element {
    rsx! {}
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum SiteRoute {
    #[layout(Site)]
    #[route("/")]
    Home {},
    #[route("/docs")]
    Docs {},
    #[nest("/docs")]
        #[layout(Docs)]
        #[route("/:section?:item")]
        DocsContent { section: String, item: String },
}

#[component]
fn Site() -> Element {
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
                            to: NavigationTarget::Internal(SiteRoute::Home {}).into(),
                        },
                        NavBarItemProps {
                            label: "Docs".to_owned(),
                            to: NavigationTarget::Internal(SiteRoute::Docs {}).into(),
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
            Outlet::<SiteRoute> { }
        }
    }
}
