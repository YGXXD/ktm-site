//  Created by 有个小小杜

use crate::assets::*;
use crate::site::*;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        document::Title { "ktm - 属于珂学家们的数学库" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: THEME_CSS }
        document::Link { rel: "stylesheet", href: STYLES_CSS }
        script { src: ONLOAD_JS }
        Router::<SiteRoute> {}
    }
}
