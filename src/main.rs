use ktm_docs::components::mark_down::MarkDown;
use ktm_docs::components::menu_list::*;
use ktm_docs::components::search_box::SearchBox;
use ktm_docs::components::top_guide::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{MediaQueryListEvent, window};
use yew::prelude::*;

const MD: &str = include_str!("../README.md");

pub fn setup_theme_mode_and_watcher() {
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
fn App() -> Html {
    setup_theme_mode_and_watcher();

    let mut sections = vec![];
    for _ in 0..12 {
        sections.push(MenuListSectionProps {
            title: "Section 1".to_owned(),
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

    html! {
        <div class="app">
            <div class="guide-area"> 
                <div class="guide-content">
                    <TopGuide
                        logo={
                            TopGuidLogoProps {
                                src: Some("logo.png".to_string()),
                                height: 50
                            }
                        } 
                        items={
                            vec![
                                TopGuideItemProps {
                                    label: "Github".to_owned(),
                                    link: "https://github.com/YGXXD/ktm".to_owned(),
                                }
                            ]
                        }
                    />
                </div>
            </div>
            <div class="docs-area">
                <aside class="docs-sidebar">
                    <div class="docs-sidebar-header">
                        <SearchBox
                            placeholder={"search documentation...".to_owned()}
                            onchange={Callback::from(|value: String| {
                            })}
                        />
                    </div>
                    <div class="docs-sidebar-content">
                        <MenuList
                            sections ={sections}
                        />
                    </div>
                </aside>   
                <main class="docs-main">
                    <MarkDown content={MD.to_string()}/>
                </main>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
