use crate::assets::*;
use crate::components::mark_down::MarkDown;
use crate::components::menu_list::*;
use crate::components::search_box::SearchBox;
use crate::components::top_guide::*;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let top_logo = Some(TopGuidLogoProps {
        src: Some(LOGO.to_string()),
        height: 50,
    });
    let top_items = {
        vec![TopGuideItemProps {
            label: "Github".to_owned(),
            link: "https://github.com/YGXXD/ktm".to_owned(),
        }]
    };

    let mut default_sections = vec![];
    for i in 0..12 {
        default_sections.push(MenuListSectionProps {
            title: "Section".to_owned() + &i.to_string(),
            items: vec![
                MenuListItemProps {
                    label: "Home".to_owned(),
                    link: "/".to_owned(),
                },
                MenuListItemProps {
                    label: "About".to_owned(),
                    link: "/about".to_owned(),
                },
                MenuListItemProps {
                    label: "Contact".to_owned(),
                    link: "/contact".to_owned(),
                },
            ],
        });
    }

    let mut sections_singal = use_signal(|| default_sections.clone());
    let search_box_oninput = move |value: String| {
        if !value.is_empty() {
            let new_sections: Vec<_> = default_sections
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
            sections_singal.set(default_sections.clone());
        }
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: THEME_CSS }
        document::Link { rel: "stylesheet", href: STYLES_CSS }
        script { src: ONLOAD_JS }
        div {
            class: "guide-area",
            div {
                class: "guide-content",
                TopGuide {
                    logo: top_logo,
                    items: top_items
                }
            }
        }
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
                MarkDown {
                    content: MD.to_string()
                }
            }
        }
    }
}
