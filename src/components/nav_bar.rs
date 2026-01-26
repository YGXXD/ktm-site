use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct NavBarLogoProps {
    pub src: Asset,
    pub height: u32,
}

#[derive(PartialEq, Clone, Props)]
pub struct NavBarItemProps {
    pub label: String,
    pub to: NavigationTarget,
}

#[component]
pub fn NavBar(logo: Option<NavBarLogoProps>, items: Vec<NavBarItemProps>) -> Element {
    rsx! {
        div {
            class: "nav-bar",
            if let Some(logo) = logo {
                img {
                    src: logo.src.clone(),
                    height: logo.height.to_string()
                }
            }
            ul {
                for (index, item) in items.iter().enumerate() {
                    if index != items.len() - 1 {
                        normal-item {
                            Link {
                                to: item.to.clone(),
                                "{item.label}"
                            }
                        }
                    }  else {
                        last-item {
                            Link {
                                to: item.to.clone(),
                                "{item.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
