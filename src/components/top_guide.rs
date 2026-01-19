use dioxus::prelude::*;

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
pub fn TopGuide(logo: Option<TopGuidLogoProps>, items: Vec<TopGuideItemProps>) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: row;
                align-items: center;
            "#,
            if let Some(logo) = logo {
                img {
                    src: logo.src.clone(),
                    height: logo.height.to_string()
                }
            }
            ul {
                style: r#"
                    display: flex;
                    flex-direction: row;
                    list-style: none;
                    align-items: center;
                    margin-left: auto;
                    margin-right: 1rem;
                "#,
                for item in items {
                    li {
                        a {
                            style: r#"
                                display: block;
                                margin-top: 1.5rem;
                                font-size: large;
                            "#,
                            href: item.link.clone(),
                            "{item.label}"
                        }
                    }
                }
            }
        }
    }
}
