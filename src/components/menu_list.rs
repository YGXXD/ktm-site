use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MenuListItemProps {
    pub label: String,
    pub link: String,
}

#[derive(Properties, PartialEq)]
pub struct MenuListSectionProps {
    pub title: String,
    pub items: Vec<MenuListItemProps>,
}

#[derive(Properties, PartialEq)]
pub struct MenuListProps {
    pub sections: Vec<MenuListSectionProps>,
}

#[styled_component(MenuList)]
pub fn menu_list(props: &MenuListProps) -> Html {
    let menu_list_style_sheet = style!(
        r#"
            h4 {
                font-weight: 600;
                margin: 0.5rem 0;
            }
            ul {
                list-style: none;
                padding: 0;
                margin: 0;
            }
            li {
                padding: 0.1rem 0;
            }
            a {
                text-decoration: none;
                display: block;
                padding: 0.15rem 0.8rem;
                border-radius: 6px;
            }
            a:hover {
                background: rgba(3, 37, 105, 0.08);
            }
        "#
    )
    .unwrap();
    html! {
        <div class={menu_list_style_sheet.get_class_name().to_string()}>
            {
                for props.sections.iter().map(move |section| {
                    html! {
                        <p>
                            <h4>{ section.title.clone() }</h4>
                            <ul>
                                {
                                    for section.items.iter().map(move |item| {
                                        html! {
                                            <li>
                                                <a href={item.link.clone()}>{ &item.label }</a>
                                            </li>
                                        }
                                    })
                                }
                            </ul>
                        </p>
                    }
                })
            }
        </div>
    }
}
