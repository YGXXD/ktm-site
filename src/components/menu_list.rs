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
pub fn MenuList(sections: Vec<MenuListSectionProps>) -> Element {
    rsx! {
        div {
            for section in sections {
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
                        for item in section.items {
                            li {
                                style: r#"
                                    list-style: none;
                                    margin-right: 1rem;
                                "#,
                                Link {
                                    style: r#"
                                        display: block;
                                        border-radius: 6px;
                                        margin: 0.05rem 0.8rem;
                                    "#,
                                    to: item.to,
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
