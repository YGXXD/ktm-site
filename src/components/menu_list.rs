use leptos::prelude::*;
use stylist::style;

#[derive(PartialEq, Clone)]
pub struct MenuListItemProps {
    pub label: String,
    pub link: String,
}

#[derive(PartialEq, Clone)]
pub struct MenuListSectionProps {
    pub title: String,
    pub items: Vec<MenuListItemProps>,
}

#[component]
pub fn MenuList(sections: Vec<MenuListSectionProps>) -> impl IntoView {
    let menu_list_style_sheet = style!(
        r#"
            p {
                margin: 0.5rem 0;
            }
            h3 {
                font-weight: 700;
                margin: 0;
            }
            li {
                list-style: none;
                margin-right: 1rem;
            }
            a {
                display: block;
                border-radius: 6px;
                margin: 0.05rem 0.8rem;
            }
        "#
    )
    .unwrap();
    view! {
        <div class={menu_list_style_sheet.get_class_name().to_string()}>
            {
                sections.iter().map(move |section| {
                    view! {
                        <p>
                            <h3>{ section.title.clone() }</h3>
                            <div>
                                {
                                    section.items.iter().map(move |item| {
                                        view! {
                                            <li>
                                                <a href={item.link.clone()}>{item.label.clone()}</a>
                                            </li>
                                        }
                                    }).collect_view()
                                }
                            </div>
                        </p>
                    }
                }).collect_view()
            }
        </div>
    }
}
