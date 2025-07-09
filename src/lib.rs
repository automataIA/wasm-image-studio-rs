// src/lib.rs
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

// Modules
pub mod components;
pub mod filters;
pub mod pages;
pub mod utils;

// Top-Level pages
use crate::pages::home::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Script per gestire il redirect da 404.html
    let restore_redirect = move || {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.session_storage() {
                if let Ok(Some(redirect)) = storage.get_item("redirect") {
                    let _ = storage.remove_item("redirect");
                    if let Ok(history) = window.history() {
                        let _ = history.replace_state_with_url(
                            &wasm_bindgen::JsValue::NULL,
                            "",
                            Some(&redirect),
                        );
                    }
                }
            }
        }
    };

    // ✅ Usa Effect::new() invece di create_effect()
    Effect::new(move |_| {
        restore_redirect();
    });

    view! {
        <html lang="en" dir="ltr" data-theme="bumblebee" />
        <Title text="WASM Image Studio" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/wasm-image-studio-rs/") view=Home />
                <Route path=path!("/404.html") view=NotFound />
            </Routes>
        </Router>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="hero min-h-screen bg-base-200">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <h1 class="text-5xl font-bold">"404"</h1>
                    <p class="py-6">"Pagina non trovata"</p>
                    <a href="/wasm-image-studio-rs/" class="btn btn-primary">
                        "Torna alla Home"
                    </a>
                </div>
            </div>
        </div>
    }
}
