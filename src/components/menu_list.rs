//  Created by 有个小小杜

use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct MenuListItemProps {
    pub label: String,
    pub to: NavigationTarget,
}

#[derive(PartialEq, Clone)]
pub struct MenuListSectionProps {
    pub title: String,
    pub items: Vec<MenuListItemProps>,
}

#[component]
pub fn MenuList(sections: Vec<MenuListSectionProps>, selected: Option<(usize, usize)>) -> Element {
    #[cfg(debug_assertions)]
    {
        dioxus::logger::tracing::debug!("menu list render");
    }
    rsx! {
        div {
            class: "menu-list",
            for (si, section) in sections.iter().enumerate() {
                section {
                    h3 { "{section.title}" }
                    div {
                        for (ii, item) in section.items.iter().enumerate() {
                            li {
                                if selected.is_some() && selected.unwrap() == (si, ii) {
                                    selected-item {
                                        Link {
                                            to: item.to.clone(),
                                            "{item.label}"
                                        }
                                    }
                                } else {
                                    normal-item {
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
        }
    }
}
