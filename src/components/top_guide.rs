use leptos::prelude::*;
use stylist::style;

#[derive(PartialEq, Clone)]
pub struct TopGuidLogoProps {
    pub src: Option<String>,
    pub height: u32,
}

#[derive(PartialEq, Clone)]
pub struct TopGuideItemProps {
    pub label: String,
    pub link: String,
}

#[component]
pub fn TopGuide(logo: Option<TopGuidLogoProps>, items: Vec<TopGuideItemProps>) -> impl IntoView {
    let top_guide_style_sheet = style!(
        r#"
            & {
                display: flex;
                flex-direction: row;
                align-items: center;
            }
            ul {
                display: flex;
                flex-direction: row;
                list-style: none;
                align-items: center;
                margin-left: auto;
                margin-right: 1rem;
            }
            a {
                display: block;
                margin-top: 1.5rem;
                font-size: large;
            }
        "#
    )
    .unwrap();

    view! {
        <div class={top_guide_style_sheet.get_class_name().to_string()}>
            {
                match logo {
                    Some(logo_data) => Some(view! {
                        <img
                            src={logo_data.src.clone()}
                            height={logo_data.height.to_string()}
                        />
                    }),
                    None => None
                }
            }
            <ul>
                {
                    items.iter().map(move |item| {
                        view! {
                            <li>
                                <a href={item.link.clone()}>{ item.label.clone() }</a>
                            </li>
                        }
                    }).collect_view()
                }
            </ul>
        </div>
    }
}
