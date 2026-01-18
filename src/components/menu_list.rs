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
    html! {
        <div class={menu_list_style_sheet.get_class_name().to_string()}>
            {
                for props.sections.iter().map(move |section| {
                    html! {
                        <p>
                            <h3>{ section.title.clone() }</h3>
                            <div>
                                {
                                    for section.items.iter().map(move |item| {
                                        html! {
                                            <li>
                                                <a href={item.link.clone()}>{ &item.label }</a>
                                            </li>
                                        }
                                    })
                                }
                            </div>
                        </p>
                    }
                })
            }
        </div>
    }
}
