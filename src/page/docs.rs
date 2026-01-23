use crate::api::*;
use crate::components::mark_down::MarkDown;
use crate::components::menu_list::*;
use crate::components::search_box::SearchBox;
use crate::site::SiteRoute;
use dioxus::prelude::*;

#[component]
pub fn DocsContent(path: String) -> Element {
    rsx! {
        MarkDown {
            path: path
        }
    }
}

#[component]
pub fn DocsDefault() -> Element {
    rsx! {
        MarkDown {
            path: None
        }
    }
}

#[component]
pub fn Docs() -> Element {
    let mut sections_singal = use_signal(|| vec![]);
    let mut default_sections_singnal = use_signal(|| vec![]);
    use_effect(move || {
        spawn(async move {
            let data = docs_menulist().await.unwrap();
            let serde_result =
                serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&data);
            if serde_result.is_err() {
                return;
            }
            let sections = serde_result
                .unwrap()
                .iter()
                .filter_map(|(key, value)| {
                    if let Some(value) = value.as_array() {
                        let items: Vec<_> = value
                            .iter()
                            .filter_map(|item| {
                                if let Some(item) = item.as_str() {
                                    Some(MenuListItemProps {
                                        label: item.to_owned(),
                                        to: NavigationTarget::Internal(SiteRoute::DocsContent {
                                            path: item.to_owned(),
                                        })
                                        .into(),
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        if items.is_empty() {
                            None
                        } else {
                            Some(MenuListSectionProps {
                                title: key.to_owned(),
                                items: items,
                            })
                        }
                    } else {
                        None
                    }
                })
                .collect();
            default_sections_singnal.set(sections);
            sections_singal.set(default_sections_singnal());
        });
    });
    let search_box_oninput = move |value: String| {
        if !value.is_empty() {
            let new_sections = default_sections_singnal()
                .iter()
                .filter_map(|section| {
                    let filtered_items: Vec<_> = section
                        .items
                        .iter()
                        .filter(|item| item.label.contains(&value))
                        .cloned()
                        .collect();
                    if !filtered_items.is_empty() {
                        Some(MenuListSectionProps {
                            title: section.title.clone(),
                            items: filtered_items,
                        })
                    } else {
                        None
                    }
                })
                .collect();
            sections_singal.set(new_sections);
        } else {
            sections_singal.set(default_sections_singnal());
        }
    };

    rsx! {
        div {
            class: "docs-area",
            aside {
                class: "docs-sidebar",
                div {
                    class: "docs-sidebar-header",
                    SearchBox {
                        placeholder: "search documentation...".to_owned(),
                        oninput: search_box_oninput
                    }
                }
                div {
                    class: "docs-sidebar-content",
                    MenuList {
                        sections: sections_singal()
                    }
                }
            }
            main {
                class: "docs-main",
                Outlet::<SiteRoute> {}
            }
        }
    }
}
