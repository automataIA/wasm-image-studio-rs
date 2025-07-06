// src/lib.rs
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
pub mod components;
pub mod filters;
pub mod pages;
pub mod utils;

// Top-Level pages
use crate::pages::home::Home;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="hero min-h-screen bg-base-200">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <h1 class="text-5xl font-bold">"404"</h1>
                    <p class="py-6">"Page not found"</p>
                    <a href="/" class="btn btn-primary">
                        "Torna alla Home"
                    </a>
                </div>
            </div>
        </div>
    }
}

/// An app router which renders the homepage and handles 404's
#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="it" attr:dir="ltr" attr:data-theme="bumblebee" />

        // sets the document title
        <Title text="WASM Image Studio - Rust WebAssembly Image Processing" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <body class="bg-base-200 min-h-screen">
            <Router>
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route path=path!("/") view=Home />
                </Routes>
            </Router>
        </body>
    }
}
