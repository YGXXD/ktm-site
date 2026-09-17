use dioxus::prelude::*;

#[component]
pub fn HomeContent() -> Element {
    rsx! {
        div {
            class: "home-area",
            section {
                class: "home-hero",
                span { class: "home-symbol", style: "top: 15%; left: 10%; font-size: 4rem; animation-duration: 7s;", "π" }
                span { class: "home-symbol", style: "top: 62%; left: 17%; font-size: 3rem; animation-duration: 9s; animation-delay: 1.2s;", "∑" }
                span { class: "home-symbol", style: "top: 18%; right: 14%; font-size: 3.5rem; animation-duration: 8s; animation-delay: 0.6s;", "√" }
                span { class: "home-symbol", style: "top: 65%; right: 10%; font-size: 4.5rem; animation-duration: 10s; animation-delay: 2s;", "∫" }
                span { class: "home-symbol", style: "top: 42%; left: 46%; font-size: 2.5rem; animation-duration: 11s; animation-delay: 3s;", "Δ" }
                h1 { class: "home-hero-title", "珂朵莉数学库" }
                p { class: "home-hero-subtitle", "kutori math" }
                p {
                    class: "home-hero-desc",
                    "现代 C++17 图形数学库：向量、矩阵、四元数、复数与仿射变换"
                }
                div {
                    class: "home-features",
                    div {
                        class: "home-feature-card",
                        h3 { class: "home-feature-title", "代码魔法" }
                        p { class: "home-feature-desc", "很久很久以前，咒语被写进了模板里——从此，错误都会在编译的黎明前，被悄悄拦下" }
                    }
                    div {
                        class: "home-feature-card",
                        h3 { class: "home-feature-title", "架构优雅" }
                        p { class: "home-feature-desc", "传说中，组件与系统立下过约定：各安其位，互不打扰——这便是静态 ECS 的童话" }
                    }
                    div {
                        class: "home-feature-card",
                        h3 { class: "home-feature-title", "高可读性" }
                        p { class: "home-feature-desc", "故事里的这套 API，直白得像睡前听过的童话——读到第一行，就猜到了美满的结局" }
                    }
                    div {
                        class: "home-feature-card",
                        h3 { class: "home-feature-title", "零依赖" }
                        p { class: "home-feature-desc", "据说有一个库，从不向任何人伸手，只等一次 include——然后，它便整个属于你" }
                    }
                    div {
                        class: "home-feature-card",
                        h3 { class: "home-feature-title", "高性能" }
                        p { class: "home-feature-desc", "在暮色降临的时分，名为 SIMD 的翅膀悄然张开——运算便如黄金妖精般掠过天际" }
                    }
                    div {
                        class: "home-feature-card",
                        h3 { class: "home-feature-title", "跨平台" }
                        p { class: "home-feature-desc", "哪怕世界终结于任何一座浮游大陆，它也会陪在你身边——编译、运行，直到最后" }
                    }
                }
            }
            footer {
                class: "home-footer",
                "MIT License · "
                a { href: "https://github.com/YGXXD/ktm", target: "_blank", rel: "noopener noreferrer", "GitHub" }
                " · Copyright © 2023-2026 "
                a { href: "https://github.com/YGXXD", target: "_blank", rel: "noopener noreferrer", "有个小小杜" }
            }
        }
    }
}
