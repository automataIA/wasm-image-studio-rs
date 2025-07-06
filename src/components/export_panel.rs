// src/components/export_panel.rs
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, HtmlCanvasElement};

#[allow(non_snake_case)]
#[component]
pub fn ExportPanel(
    canvas_ref: NodeRef<leptos::html::Canvas>,
    _image_src: Signal<Option<String>>,
) -> impl IntoView {
    let (export_format, set_export_format) = signal("PNG".to_string());
    let (filename, set_filename) = signal("processed-image".to_string());
    let (is_exporting, set_is_exporting) = signal(false);

    // Handle export functionality
    let handle_export = move |_| {
        set_is_exporting.set(true);

        // Get the canvas element
        if let Some(canvas_element) = canvas_ref.get() {
            let canvas: HtmlCanvasElement = canvas_element;

            // Convert canvas to data URL based on selected format
            let data_url = match export_format.get().as_str() {
                "JPEG" => canvas
                    .to_data_url_with_type("image/jpeg")
                    .unwrap_or_default(),
                "WEBP" => canvas
                    .to_data_url_with_type("image/webp")
                    .unwrap_or_default(),
                _ => canvas.to_data_url().unwrap_or_default(), // Default to PNG
            };

            // Create a temporary link to trigger download
            let window = web_sys::window().expect("no global window exists");
            let document = window.document().expect("no document exists");
            let a = document.create_element("a").unwrap();
            let a: HtmlAnchorElement = a.dyn_into().unwrap();

            a.set_href(&data_url);
            a.set_download(&format!(
                "{}.{}",
                filename.get(),
                export_format.get().to_lowercase()
            ));
            a.click();
        }

        set_is_exporting.set(false);
    };

    view! {
        <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
                <h2 class="card-title">
                    <span class="text-2xl">"💾"</span>
                    "Export Image"
                </h2>

                <div class="form-control w-full max-w-xs">
                    <label class="label">
                        <span class="label-text font-semibold">"Format:"</span>
                    </label>
                    <select
                        class="select select-bordered w-full max-w-xs"
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            set_export_format.set(value);
                        }
                    >
                        <option value="PNG" selected=move || export_format.get() == "PNG">
                            "PNG"
                        </option>
                        <option value="JPEG" selected=move || export_format.get() == "JPEG">
                            "JPEG"
                        </option>
                        <option value="WEBP" selected=move || export_format.get() == "WEBP">
                            "WEBP"
                        </option>
                    </select>
                </div>

                <div class="form-control w-full max-w-xs">
                    <label class="label">
                        <span class="label-text font-semibold">"Filename:"</span>
                    </label>
                    <input
                        type="text"
                        class="input input-bordered w-full max-w-xs"
                        prop:value=move || filename.get()
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_filename.set(value);
                        }
                    />
                </div>

                <div class="card-actions justify-end mt-4">
                    <button
                        class=move || {
                            let base = "btn btn-primary btn-md normal-case w-full sm:w-40";
                            if is_exporting.get() {
                                format!("{} loading", base)
                            } else {
                                base.to_string()
                            }
                        }
                        on:click=handle_export
                        disabled=move || is_exporting.get()
                    >
                        {move || {
                            if is_exporting.get() {
                                view! { <span>"Exporting..."</span> }.into_any()
                            } else {
                                view! {
                                    <span class="mr-2">"⬇️"</span>
                                    <span>"Export Image"</span>
                                }
                                .into_any()
                            }
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}
