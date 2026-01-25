use crate::api::post_docs_content;
use crate::assets::DOCS_CONFIG;
use crate::components::mark_down::MarkDown;
use crate::components::menu_list::*;
use crate::components::search_box::SearchBox;
use crate::site::SiteRoute;
use dioxus::prelude::*;

#[component]
pub fn DocsContent(section: String, item: String) -> Element {
    let resource = use_resource(use_reactive((&(section, item),), |(params,)| async {
        #[cfg(debug_assertions)]
        {
            dioxus::logger::tracing::debug!(
                "post_docs_content: section: {}, item: {}",
                params.0.clone(),
                params.1.clone()
            );
        }
        post_docs_content(params.0, params.1).await
    }));

    rsx! {
        MarkDown {
            content: match &*resource.read() {
                Some(result) => match result {
                    Ok(content) => Some(content.clone()),
                    Err(_) => Some("# 找不到文档(Not Find Docs)!!!".to_string())
                },
                None => None,
            }
        }
    }
}

async fn load_default_sections() -> Vec<MenuListSectionProps> {
    let config_bytes = dioxus::asset_resolver::read_asset_bytes(&DOCS_CONFIG)
        .await
        .unwrap();
    let json_result = serde_json::from_slice::<Vec<serde_json::Value>>(&config_bytes);
    match json_result {
        Ok(json_result) => json_result
            .iter()
            .filter_map(|section_data| {
                if let Some(section_data) = section_data.as_object() {
                    let title = section_data.get("title");
                    let items = section_data.get("items");
                    if title.is_none() || items.is_none() {
                        return None;
                    }
                    let title = title.unwrap().as_str();
                    let items = items.unwrap().as_array();
                    if title.is_none() || items.is_none() {
                        return None;
                    }
                    let items: Vec<_> = items
                        .unwrap()
                        .iter()
                        .filter_map(|item| {
                            if let Some(item) = item.as_str() {
                                Some(MenuListItemProps {
                                    label: item.to_owned(),
                                    to: NavigationTarget::Internal(SiteRoute::DocsContent {
                                        section: title.unwrap().to_owned(),
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
                            title: title.unwrap().to_owned(),
                            items: items,
                        })
                    }
                } else {
                    None
                }
            })
            .collect(),
        _ => vec![],
    }
}

fn filter_section_item(
    sections: &Vec<MenuListSectionProps>,
    filter_str: String,
) -> Vec<MenuListSectionProps> {
    if !filter_str.is_empty() {
        let new_sections = sections
            .iter()
            .filter_map(|section| {
                let filtered_items: Vec<_> = section
                    .items
                    .iter()
                    .filter(|item| item.label.contains(&filter_str))
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
        new_sections
    } else {
        sections.clone()
    }
}

fn find_section_item_idx(
    sections: &Vec<MenuListSectionProps>,
    find_str: Option<(String, String)>,
) -> Option<(usize, usize)> {
    if let Some((section_str, item_str)) = find_str {
        if let Some(si) = sections
            .iter()
            .position(|section| section.title == section_str)
        {
            if let Some(ii) = sections[si]
                .items
                .iter()
                .position(|item| item.label == item_str)
            {
                Some((si, ii))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

#[component]
pub fn Docs() -> Element {
    let mut sections_signal: Signal<Vec<MenuListSectionProps>> = use_signal(|| vec![]);
    let mut selected_idx_signal: Signal<Option<(usize, usize)>> = use_signal(|| None);

    let default_sections = use_resource(move || async move {
        let sections = load_default_sections().await;
        #[cfg(debug_assertions)]
        {
            dioxus::logger::tracing::debug!("resource_load: sections_len: {}", sections.len());
        }
        if !sections.is_empty() {
            sections_signal.set(sections.clone());
        }
        sections
    });

    use_effect(move || {
        let route = use_route::<SiteRoute>();
        let sections = sections_signal();
        #[cfg(debug_assertions)]
        {
            dioxus::logger::tracing::debug!(
                "route_effect: route: {}, sections_len: {}",
                route.clone(),
                sections.len()
            );
        }
        let mut selected_idx = None;
        match route {
            SiteRoute::DocsContent { section, item } => {
                selected_idx = find_section_item_idx(&sections, Some((section, item)));
            }
            _ => {
                if !sections.is_empty() {
                    selected_idx = Some((0, 0));
                }
            }
        }
        let selected_str = if let Some(selected_idx) = selected_idx {
            Some((
                sections[selected_idx.0].title.clone(),
                sections[selected_idx.0].items[selected_idx.1].label.clone(),
            ))
        } else {
            None
        };
        #[cfg(debug_assertions)]
        {
            dioxus::logger::tracing::debug!(
                "route_effect: selected_str: {}, selected_idx: {}",
                match selected_str.clone() {
                    None => "none".to_string(),
                    Some((s, i)) => s + "-" + &i,
                },
                match selected_idx {
                    None => "none".to_string(),
                    Some((s, i)) => s.to_string() + "-" + &i.to_string(),
                }
            );
        }
        selected_idx_signal.set(selected_idx);
    });

    let search_box_oninput = move |value: String| {
        let default_sections = &*default_sections.read();
        if let Some(default_sections) = default_sections {
            let selected_str = if let Some(selected_idx) = selected_idx_signal() {
                let sections = sections_signal();
                Some((
                    sections[selected_idx.0].title.clone(),
                    sections[selected_idx.0].items[selected_idx.1].label.clone(),
                ))
            } else {
                None
            };
            let filter_sections = filter_section_item(&default_sections, value);
            let selected_idx = find_section_item_idx(&filter_sections, selected_str);

            #[cfg(debug_assertions)]
            {
                dioxus::logger::tracing::debug!(
                    "search_box_oninput: filter_sections_len: {}",
                    filter_sections.len()
                );
            }
            sections_signal.set(filter_sections);
            selected_idx_signal.set(selected_idx);
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
                        sections: sections_signal(),
                        selected: selected_idx_signal()
                    }
                }
            }
            main {
                class: "docs-main",
                match use_route::<SiteRoute>() {
                    SiteRoute::Docs {} => {
                        if let Some(section_idx) = selected_idx_signal() {
                            rsx!(
                                DocsContent {
                                    section: sections_signal()[section_idx.0].title.clone(),
                                    item: sections_signal()[section_idx.0].items[section_idx.1].label.clone()
                                }
                            )
                        } else {
                            rsx!()
                        }
                    },
                    _ => rsx!(
                        Outlet::<SiteRoute> {}
                    )
                }
            }
        }
    }
}
