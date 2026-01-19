mod components;

use components::mark_down::MarkDown;
use components::menu_list::*;
use components::search_box::SearchBox;
use components::top_guide::*;
use leptos::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{MediaQueryListEvent, window};

const MD: &str = include_str!("../README.md");

fn setup_theme_mode_and_watcher() {
    if let Some(win) = window() {
        let media_query = "(prefers-color-scheme: dark)";
        let mq_list = win.match_media(media_query).unwrap().unwrap();
        let document = win.document().unwrap();
        let _ = document.document_element().unwrap().set_attribute(
            "data-theme",
            if mq_list.matches() { "dark" } else { "light" },
        );
        let closure: Closure<dyn FnMut(MediaQueryListEvent)> =
            Closure::new(Box::new(move |event: MediaQueryListEvent| {
                let _ = document
                    .document_element()
                    .unwrap()
                    .set_attribute("data-theme", if event.matches() { "dark" } else { "light" });
            }) as Box<dyn FnMut(MediaQueryListEvent)>);
        let _ =
            mq_list.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

#[component]
pub fn App() -> impl IntoView {
    setup_theme_mode_and_watcher();

    let top_logo = Some(TopGuidLogoProps {
        src: Some("logo.png".to_string()),
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

    let (sections, set_sections) = signal(default_sections.clone());
    let search_box_oninput = move |value: String| {
        web_sys::console::log_1(&"search docs: ".into());
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
            set_sections.set(new_sections);
        } else {
            set_sections.set(default_sections.clone());
        }
    };

    view! {
        <div class="app">
            <div class="guide-area">
                <div class="guide-content">
                    <TopGuide
                        logo={top_logo}
                        items={top_items}
                    />
                </div>
            </div>
            <div class="docs-area">
                <aside class="docs-sidebar">
                    <div class="docs-sidebar-header">
                        <SearchBox
                            placeholder={"search documentation...".to_owned()}
                            oninput={search_box_oninput}
                        />
                    </div>
                    <div class="docs-sidebar-content">
                        {
                            move || view! {
                                <MenuList
                                    sections={sections.get()}
                                />
                            }
                        }
                    </div>
                </aside>
                <main class="docs-main">
                    <MarkDown content={MD.to_string()}/>
                </main>
            </div>
        </div>
    }
}
