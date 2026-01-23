use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct NavBarLogoProps {
    pub src: Asset,
    pub height: u32,
}

#[derive(PartialEq, Clone, Props)]
pub struct NavBarItemProps {
    pub label: String,
    pub to: NavigationTarget,
}

#[component]
pub fn NavBar(logo: Option<NavBarLogoProps>, items: Vec<NavBarItemProps>) -> Element { 
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: row;
                width: 100%;
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
                for (index, item) in items.iter().enumerate() {
                    li {
                        style: {
                            if index != items.len() - 1 {r#"
                                margin-top: 1.5rem;
                                margin-right: 1rem;
                                font-size: large;
                            "#} else {r#"
                                margin-top: 1.5rem;
                                font-size: large;
                            "#}
                        },
                        Link {
                            to: item.to.clone(),
                            "{item.label}"
                        }
                    }
                }
            }
        }
    }
}
