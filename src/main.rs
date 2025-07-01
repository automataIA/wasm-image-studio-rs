// src/main.rs
use leptos::prelude::*;
use wasm_image_studio_rs::App;

fn main() {
    // Set up logging and panic hook for WebAssembly
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log::info!("WASM Image Studio initializing...");

    // Mount the application to the body
    mount_to_body(|| {
        view! { <App /> }
    });

    log::info!("WASM Image Studio mounted successfully");
}
