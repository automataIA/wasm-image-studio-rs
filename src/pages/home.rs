// src/pages/home.rs

use crate::components::{DragDrop, ExportPanel, FilterControls, ImageCanvas};
use crate::filters::FilterType;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

    view! {
        <div class="min-h-screen bg-gray-100">
            <h1 class="text-3xl font-bold text-center py-8">"WASM Image Studio"</h1>

            <div class="container mx-auto px-4">
                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                    <div class="space-y-6">
                        <DragDrop on_file_upload=handle_file_upload />

                        <FilterControls
                            on_filter_change=handle_filter_change
                            intensity=filter_intensity
                            // ✅ Use the Callback
                            on_intensity_change=handle_intensity_change
                        />
                    </div>

                    <div class="lg:col-span-2">
                        <ImageCanvas
                            image_src=image_src.into()
                            active_filter=active_filter
                            filter_intensity=filter_intensity
                            node_ref=canvas_ref
                        />

                        <ExportPanel canvas_ref=canvas_ref _image_src=image_src.into() />
                    </div>
                </div>
            </div>
        </div>
    }
}
