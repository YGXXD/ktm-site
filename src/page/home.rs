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
                        h3 { class: "home-feature-title", "Header-Only" }
                        p { class: "home-feature-desc", "单头文件引入即用，无需编译链接" }
                    }
                    div {
                        class: "home-feature-card",
                        h3 { class: "home-feature-title", "零依赖" }
                        p { class: "home-feature-desc", "仅依赖 C++ 标准库，无任何第三方库" }
                    }
                    div {
                        class: "home-feature-card",
                        h3 { class: "home-feature-title", "类型安全" }
                        p { class: "home-feature-desc", "C++17 约束模板参数，编译期拦截误用" }
                    }
                    div {
                        class: "home-feature-card",
                        h3 { class: "home-feature-title", "SIMD 加速" }
                        p { class: "home-feature-desc", "使用 SSE / NEON / WASM SIMD 指令集优化" }
                    }
                    div {
                        class: "home-feature-card",
                        h3 { class: "home-feature-title", "跨平台" }
                        p { class: "home-feature-desc", "Windows / macOS / Linux / WebAssembly 开箱即用" }
                    }
                    div {
                        class: "home-feature-card",
                        h3 { class: "home-feature-title", "极简的 API" }
                        p { class: "home-feature-desc", "统一直观的命名约定，学习和使用成本低" }
                    }
                }
            }
            footer {
                class: "home-footer",
                "MIT License · "
                a { href: "https://github.com/YGXXD/ktm", target: "_blank", rel: "noopener noreferrer", "GitHub" }
                " · Copyright © 2023-2026 有个小小杜"
            }
        }
    }
}
