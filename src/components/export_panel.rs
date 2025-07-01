// src/components/export_panel.rs
use leptos::prelude::*;
use tailwind_fuse::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;

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
        if let Some(canvas) = canvas_ref.get() {
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
        <div class=tw_join!(
            "export-panel", "bg-white", "p-6", "rounded-lg", "shadow-sm", "border", "border-gray-100"
        )>
            <h3 class=tw_join!("text-lg", "font-bold", "mb-4")>"💾 Export Image"</h3>

            <div class=tw_join!("export-field", "mb-4")>
                <label
                    for="format-select"
                    class=tw_join!("block", "text-sm", "font-medium", "text-gray-700", "mb-1")
                >
                    "Format:"
                </label>
                <select
                    id="format-select"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        set_export_format.set(value);
                    }
                    prop:value=move || export_format.get()
                    class=tw_join!(
                        "block", "w-full", "rounded-md", "border-gray-300", "shadow-sm",
                        "focus:border-blue-500", "focus:ring-blue-500", "sm:text-sm",
                        "p-2", "border"
                    )
                >
                    <option value="PNG">"PNG"</option>
                    <option value="JPEG">"JPEG"</option>
                    <option value="WEBP">"WEBP"</option>
                </select>
            </div>

            <div class=tw_join!("export-field", "mb-6")>
                <label
                    for="filename-input"
                    class=tw_join!("block", "text-sm", "font-medium", "text-gray-700", "mb-1")
                >
                    "Filename:"
                </label>
                <input
                    id="filename-input"
                    type="text"
                    prop:value=move || filename.get()
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        set_filename.set(value);
                    }
                    placeholder="processed-image"
                    class=tw_join!(
                        "block", "w-full", "rounded-md", "border-gray-300", "shadow-sm",
                        "focus:border-blue-500", "focus:ring-blue-500", "sm:text-sm",
                        "p-2", "border"
                    )
                />
            </div>

            <button
                class=move || {
                    tw_merge!(
                        "w-full", "bg-blue-500", "text-white", "font-bold", "py-2", "px-4", "rounded",
                        "hover:bg-blue-700", "focus:outline-none", "focus:ring-2", "focus:ring-blue-500",
                        "focus:ring-offset-2", "disabled:opacity-50", "disabled:cursor-not-allowed",
                        if is_exporting.get() { "opacity-75" } else { "" }
                    )
                }
                on:click=handle_export
                disabled=move || is_exporting.get()
            >
                {move || {
                    if is_exporting.get() {
                        view! {
                            <span class=tw_join!("flex", "items-center", "justify-center")>
                                <svg
                                    class=tw_join!(
                                        "animate-spin", "-ml-1", "mr-3", "h-5", "w-5", "text-white"
                                    )
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                >
                                    <circle
                                        class="opacity-25"
                                        cx="12"
                                        cy="12"
                                        r="10"
                                        stroke="currentColor"
                                        stroke-width="4"
                                    ></circle>
                                    <path
                                        class="opacity-75"
                                        fill="currentColor"
                                        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                    ></path>
                                </svg>
                                "Exporting..."
                            </span>
                        }
                            .into_any()
                    } else {
                        view! { <span>"⬇️ Export Image"</span> }.into_any()
                    }
                }}
            </button>
        </div>
    }
}
