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
pub fn MenuList(
    sections: Vec<MenuListSectionProps>,
    selected: Option<(usize, usize)>,
) -> Element {
    #[cfg(debug_assertions)]
    {
        dioxus::logger::tracing::debug!("menu list render");
    }
    rsx! {
        div {
            for (si, section) in sections.iter().enumerate() {
                div {
                    style: "margin: 0.5rem 0;",
                    h3 {
                        style: r#"
                            font-weight: 700;
                            margin: 0;
                        "#,
                        "{section.title}"
                    }
                    div {
                        for (ii, item) in section.items.iter().enumerate() {
                            li {
                                style: r#"
                                    list-style: none;
                                    margin-right: 1rem;
                                "#,
                                Link {
                                    style: if selected.is_some() && selected.unwrap() == (si, ii) {
                                        r#"
                                            display: block;
                                            border-radius: 6px;
                                            margin: 0.05rem 0.8rem;
                                            pointer-events: none;
                                            text-decoration: underline;
                                        "#
                                    } else {
                                        r#"
                                            display: block;
                                            border-radius: 6px;
                                            margin: 0.05rem 0.8rem;
                                            pointer-events: auto;
                                            text-decoration: none;
                                        "#
                                    },
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
