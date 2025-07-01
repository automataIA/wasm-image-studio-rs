// src/components/app_layout.rs
use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn AppLayout(children: Children) -> impl IntoView {
    view! {
        <div class="app-container">
            <header class="app-header">
                <h1>"🖼️ WASM Image Studio 🖼️"</h1>
                <p>"High-performance image processing with Rust and WebAssembly"</p>
            </header>
            <main class="main-content">{children()}</main>
        </div>
    }
}
