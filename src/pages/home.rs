// src/pages/home.rs

use crate::components::{DragDrop, ExportPanel, FilterControls, ImageCanvas};
use crate::filters::FilterType;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

/// WASM Image Studio - Home Page
#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {
    // Create signals using the new Leptos 0.8 API
    let (image_src, set_image_src) = signal(None::<String>);
    let (active_filter, set_active_filter) = signal(None::<FilterType>);
    let (filter_intensity, set_filter_intensity) = signal(50u8);

    // Canvas reference for export functionality
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    // Handle file upload - create Callback explicitly
    let handle_file_upload = Callback::new(move |file: web_sys::File| {
        web_sys::console::log_1(
            &format!(
                "File received in Home component: {}, type: {}, size: {} bytes",
                file.name(),
                file.type_(),
                file.size()
            )
            .into(),
        );

        // Create a FileReader to read the file as data URL
        let file_reader = web_sys::FileReader::new().expect("Failed to create FileReader");

        // Add error handler
        let error_handler = {
            let file_reader = file_reader.clone();
            let error_closure = Closure::wrap(Box::new(move |e: web_sys::ProgressEvent| {
                web_sys::console::error_1(&format!("Error reading file: {e:?}").into());
            }) as Box<dyn FnMut(_)>);
            file_reader.set_onerror(Some(error_closure.as_ref().unchecked_ref()));
            error_closure
        };

        let onload_callback = {
            let file_reader = file_reader.clone();
            let file_name = file.name();
            Closure::wrap(Box::new(move |_e: web_sys::Event| {
                web_sys::console::log_1(
                    &format!("FileReader onload triggered for: {file_name}").into(),
                );
                match file_reader.result() {
                    Ok(result) => {
                        if let Some(data_url) = result.as_string() {
                            web_sys::console::log_1(&"Setting image_src with data URL".into());
                            set_image_src.set(Some(data_url));
                        } else {
                            web_sys::console::error_1(
                                &"Failed to convert FileReader result to string".into(),
                            );
                        }
                    }
                    Err(e) => {
                        web_sys::console::error_1(
                            &format!("Error getting FileReader result: {e:?}").into(),
                        );
                    }
                }
            }) as Box<dyn FnMut(_)>)
        };

        file_reader.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
        web_sys::console::log_1(&"Starting to read file as data URL".into());

        if let Err(e) = file_reader.read_as_data_url(&file) {
            web_sys::console::error_1(&format!("Error calling readAsDataURL: {e:?}").into());
        }

        // We need to keep these closures alive until the operation completes
        std::mem::forget(onload_callback);
        std::mem::forget(error_handler);
    });

    // Handle filter change callback
    let handle_filter_change = Callback::new(move |filter: FilterType| {
        set_active_filter.set(Some(filter));
    });

    // Add this callback for intensity
    let handle_intensity_change = Callback::new(move |value: u8| {
        set_filter_intensity.set(value);
    });

    // Theme state - initialize from localStorage or system preference
    let (dark_mode, set_dark_mode) = signal({
        // Try to get from localStorage first
        let stored = window()
            .and_then(|w| w.local_storage().ok()?)
            .and_then(|storage| storage.get_item("theme").ok()?)
            .map(|theme| theme == "dark");

        // Fall back to darkMode or system preference
        stored.unwrap_or_else(|| {
            window()
                .and_then(|w| w.local_storage().ok()?)
                .and_then(|storage| storage.get_item("darkMode").ok()?)
                .and_then(|mode| mode.parse::<bool>().ok())
                .unwrap_or_else(|| {
                    window()
                        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok()?)
                        .and_then(|media| media.matches().then_some(true))
                        .unwrap_or(false)
                })
        })
    });

    // Sync theme with the signal
    Effect::new(move |_| {
        let is_dark = dark_mode.get();
        let theme = if is_dark { "dark" } else { "bumblebee" };

        if let Some(html) = document().document_element() {
            let _ = html.set_attribute("data-theme", theme);
            // Keep the dark class for compatibility with existing CSS
            if is_dark {
                let _ = html.class_list().add_1("dark");
            } else {
                let _ = html.class_list().remove_1("dark");
            }
        }

        // Save preference to localStorage
        if let Some(local_storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
            let _ = local_storage.set_item("darkMode", &is_dark.to_string());
            let _ = local_storage.set_item("theme", theme);
        }
    });

    // Toggle theme function - now just toggles the signal
    let toggle_theme = move |_| {
        let new_theme = if dark_mode.get() { "bumblebee" } else { "dark" };
        set_dark_mode.update(|d| *d = !*d);

        // Update data-theme attribute instead of class
        if let Some(html) = document().document_element() {
            let _ = html.set_attribute("data-theme", new_theme);
        }

        // Save preference to localStorage
        if let Some(local_storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
            let _ = local_storage.set_item("darkMode", &(!dark_mode.get()).to_string());
            let _ = local_storage.set_item("theme", new_theme);
        }
    };

    view! {
        <div class="min-h-screen bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-gray-100">
            <div class="relative flex items-center justify-between px-8 py-4 shadow-sm bg-white dark:bg-gray-800">
                <div class="flex-1"></div>
                {}
                <div class="flex items-center justify-center flex-1">
                    <img src="logo.png" alt="WASM Image Studio Logo" class="h-12 w-12 mr-4" />
                    <h1 class="text-3xl font-bold">"WASM Image Studio"</h1>
                </div>
                <div class="flex-1 flex justify-end">
                    <button
                        on:click=move |ev| {
                            ev.prevent_default();
                            toggle_theme(ev);
                        }
                        class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 dark:focus:ring-offset-gray-800"
                        aria-label="Toggle dark mode"
                    >
                        <Show
                            when=move || !dark_mode.get()
                            fallback=move || {
                                view! {
                                    // Moon icon (for dark mode)
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="h-6 w-6"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke="currentColor"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
                                        />
                                    </svg>
                                }
                            }
                        >
                            // Sun icon (for light mode)
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                                />
                            </svg>
                        </Show>
                    </button>
                </div>
            </div>

            <div class="container mx-auto px-4 pt-8 pb-12">
                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                    <div class="space-y-6">
                        <DragDrop on_file_upload=handle_file_upload />

                        <FilterControls
                            on_filter_change=handle_filter_change
                            intensity=filter_intensity
                            // ✅ Use the Callback
                            on_intensity_change=handle_intensity_change
                        />

                        <ExportPanel canvas_ref=canvas_ref _image_src=image_src.into() />
                    </div>

                    <div class="lg:col-span-2">
                        <ImageCanvas
                            image_src=image_src.into()
                            active_filter=active_filter
                            filter_intensity=filter_intensity
                            node_ref=canvas_ref
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}
