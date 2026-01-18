use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TopGuidLogoProps {
    pub src: Option<String>,
    pub height: u32,
}

#[derive(Properties, PartialEq)]
pub struct TopGuideItemProps {
    pub label: String,
    pub link: String,
}

#[derive(Properties, PartialEq)]
pub struct TopGuideProps {
    pub logo: Option<TopGuidLogoProps>,
    pub items: Vec<TopGuideItemProps>,
}

#[styled_component(TopGuide)]
pub fn top_guide(props: &TopGuideProps) -> Html {
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
    html! {
        <div class={top_guide_style_sheet.get_class_name().to_string()}>
            {
                if let Some(logo_data) = &props.logo {
                    html! {
                        <img 
                            src={logo_data.src.clone()}
                            height={logo_data.height.to_string()}
                        />
                    }
                } else {
                    html! {}
                }
            }
            <ul>
                {
                    for props.items.iter().map(move |item| {
                        html! {
                            <li>
                                <a href={item.link.clone()}>{ &item.label }</a>
                            </li>
                        }
                    })
                }
            </ul>
        </div>
    }
}
