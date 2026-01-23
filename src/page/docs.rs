use crate::api::*;
use crate::components::mark_down::MarkDown;
use crate::components::menu_list::*;
use crate::components::search_box::SearchBox;
use crate::site::SiteRoute;
use dioxus::prelude::*;

#[derive(Clone)]
struct DefaultSections(Signal<Vec<MenuListSectionProps>>);

#[component]
pub fn DocsContent(section: String, item: String) -> Element {
    let resource = use_resource(use_reactive((&(section, item),), |(params,)| async {
        docs_content(params.0, params.1).await
    }));

    rsx! {
        MarkDown {
            content: match &*resource.read() {
                Some(result) => match result {
                    Ok(content) => Some(content.clone()),
                    Err(_) => None,
                },
                None => None,
            }
        }
    }
}

#[component]
pub fn DocsDefault() -> Element {
    let default_sections_signal = use_context::<DefaultSections>().0;
    let resource = use_resource(move || async move {
        let default_sections = default_sections_signal();
        let section_item = if default_sections.len() > 0 {
            let mut result = None;
            for section in default_sections.iter() {
                if section.items.len() > 0 {
                    result = Some((
                        section.title.clone(),
                        section.items.get(0).unwrap().label.clone(),
                    ));
                    break;
                }
            }
            result
        } else {
            None
        };
        match section_item {
            Some((section, item)) => docs_content(section, item).await,
            None => Err(ServerFnError::Response("No Docs Found".to_string())),
        }
    });
    rsx! {
        MarkDown {
            content: match &*resource.read() {
                Some(result) => match result {
                    Ok(content) => Some(content.clone()),
                    Err(_) => None,
                },
                None => None,
            }
        }
    }
}

#[component]
pub fn Docs() -> Element {
    let mut default_sections_signal = use_signal(|| vec![]);
    let mut sections_singal = use_signal(|| vec![]);
    use_effect(move || {
        spawn(async move {
            let data = docs_menulist().await;
            if data.is_err() {
                return;
            }
            let json_result =
                serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&data.unwrap());
            if json_result.is_err() {
                return;
            }
            let sections: Vec<_> = json_result
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
                                            section: key.to_owned(),
                                            item: item.to_owned(),
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
            default_sections_signal.set(sections.clone());
            sections_singal.set(sections);
        });
    });

    let search_box_oninput = move |value: String| {
        if !value.is_empty() {
            let new_sections = default_sections_signal()
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
            sections_singal.set(default_sections_signal());
        }
    };
    use_context_provider(|| DefaultSections(default_sections_signal));

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
