// src/components/drag_drop.rs
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};

/// DragDrop component for handling image uploads
#[allow(non_snake_case)]
#[component]
pub fn DragDrop(
    #[prop(into)] on_file_upload: Callback<web_sys::File>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let (dragging, set_dragging) = signal(false);
    let input_ref = NodeRef::<leptos::html::Input>::new();

    // Handle drag events
    let on_drag_over = move |ev: DragEvent| {
        ev.prevent_default();
        set_dragging.set(true);
    };

    let on_drag_leave = move |ev: DragEvent| {
        ev.prevent_default();
        set_dragging.set(false);
    };

    let on_drop = move |ev: DragEvent| {
        ev.prevent_default();
        set_dragging.set(false);

        if let Some(data_transfer) = ev.data_transfer() {
            if let Some(files) = data_transfer.files() {
                handle_files(files, on_file_upload);
            }
        }
    };

    // Handle file input change
    let on_file_change = move |ev: Event| {
        if let Some(input) = event_target::<HtmlInputElement>(&ev) {
            if let Some(files) = input.files() {
                handle_files(files, on_file_upload);
            }
        }
    };

    // Handle click on the drop area
    let on_click_area = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        if let Some(input) = input_ref.get_untracked() {
            let html_input: HtmlInputElement = input;
            html_input.click();
        }
    };

    // Prevent click propagation from the hidden input
    let on_input_click = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
    };

    view! {
        <div class=format!("card bg-base-100 shadow-xl {}", class.unwrap_or(""))>
            <div class="card-body">
                <div
                    class=move || {
                        if dragging.get() {
                            "border-2 border-dashed border-primary bg-primary/10 rounded-lg p-8 text-center cursor-pointer transition-all duration-200 hover:bg-primary/20"
                        } else {
                            "border-2 border-dashed border-base-300 bg-base-200 rounded-lg p-8 text-center cursor-pointer transition-all duration-200 hover:bg-base-300"
                        }
                    }
                    on:dragover=on_drag_over
                    on:dragleave=on_drag_leave
                    on:drop=on_drop
                    on:click=on_click_area
                >
                    <div class="flex flex-col items-center gap-4">
                        <div class="text-4xl">
                            {move || if dragging.get() { "📁" } else { "🖼️" }}
                        </div>
                        <div class="text-lg font-semibold text-base-content">
                            "Drag & drop an image here"
                        </div>
                        <div class="text-sm text-base-content/70">
                            "or click to browse files (PNG, JPG, WEBP)"
                        </div>
                        <button class="btn btn-primary btn-md normal-case w-40">"Select File"</button>
                    </div>
                </div>

                <input
                    type="file"
                    accept="image/*"
                    class="hidden"
                    node_ref=input_ref
                    on:change=on_file_change
                    on:click=on_input_click
                />
            </div>
        </div>
    }
}

// Helper function to handle files from input or drop
fn handle_files(files: FileList, on_file_upload: Callback<web_sys::File>) {
    web_sys::console::log_1(&"handle_files called".into());

    if files.length() == 0 {
        web_sys::console::error_1(&"No files provided to handle_files".into());
        return;
    }

    if let Some(file) = files.get(0) {
        web_sys::console::log_1(
            &format!(
                "Processing file: {}, type: {}, size: {} bytes",
                file.name(),
                file.type_(),
                file.size()
            )
            .into(),
        );

        // Check if it's an image
        if file.type_().starts_with("image/") {
            web_sys::console::log_1(&"File is an image, proceeding with upload".into());
            on_file_upload.run(file);
        } else {
            let error_msg = format!("Uploaded file is not an image. Type: {}", file.type_());
            web_sys::console::error_1(&error_msg.into());
            // TODO: Show error message to user using toast notification
        }
    } else {
        web_sys::console::error_1(&"Failed to get file at index 0".into());
    }
}

// Helper function to get the target of an event as a specific type
fn event_target<T>(event: &Event) -> Option<T>
where
    T: JsCast,
{
    event
        .target()
        .and_then(|target| target.dyn_into::<T>().ok())
}
