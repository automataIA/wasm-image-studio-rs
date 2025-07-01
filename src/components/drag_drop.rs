// src/components/drag_drop.rs
use leptos::prelude::*;
use tailwind_fuse::*;
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};

/// DragDrop component for handling image uploads
///
/// This component provides both drag-and-drop and file input functionality
/// for uploading images to the application.
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
                if files.length() > 0 {
                    if let Some(file) = files.get(0) {
                        if file.type_().starts_with("image/") {
                            on_file_upload.run(file);
                        }
                    }
                }
            }
        }
    };

    // Handle file input change
    let on_file_change = move |ev: Event| {
        let input = event_target::<HtmlInputElement>(&ev);
        if let Some(files) = input.files() {
            handle_files(files, on_file_upload);
        }
    };

    // Handle click on the drop area
    let on_click_area = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        if let Some(input) = input_ref.get_untracked() {
            input.click();
        }
    };

    // Prevent click propagation from the hidden input
    let on_input_click = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
    };

    view! {
        // Card replacement using div with Tailwind classes
        <div
            class=move || {
                tw_merge!(
                    "border-2", "border-dashed", "transition-colors", "rounded-lg", "bg-white", "shadow-sm",
                    if dragging.get() {
                        "border-blue-500 bg-blue-50"
                    } else {
                        "border-gray-300 hover:border-blue-400"
                    },
                    class.unwrap_or("")
                )
            }
            on:dragover=on_drag_over
            on:dragleave=on_drag_leave
            on:drop=on_drop
            on:click=on_click_area
        >
            // CardContent replacement using div with Tailwind classes
            <div class=tw_join!(
                "flex", "flex-col", "items-center", "justify-center", "p-8", "text-center", "cursor-pointer"
            )>
                <div class=tw_join!("flex", "flex-col", "items-center", "gap-3")>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="48"
                        height="48"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class=tw_join!("w-12", "h-12", "text-gray-500")
                    >
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                        <polyline points="17 8 12 3 7 8"></polyline>
                        <line x1="12" y1="3" x2="12" y2="15"></line>
                    </svg>
                    <div class=tw_join!("space-y-1")>
                        <p class=tw_join!(
                            "text-sm", "font-medium", "text-gray-900"
                        )>"Drag & drop an image here"</p>
                        <p class=tw_join!(
                            "text-xs", "text-gray-500"
                        )>"or click to browse files (PNG, JPG, WEBP)"</p>
                    </div>
                    // Button replacement using button with Tailwind classes
                    <button
                        r#type="button"
                        class=tw_join!(
                            "mt-2", "inline-flex", "items-center", "px-3", "py-2", "border", "border-gray-300",
                            "shadow-sm", "text-sm", "leading-4", "font-medium", "rounded-md", "text-gray-700",
                            "bg-white", "hover:bg-gray-50", "focus:outline-none", "focus:ring-2",
                            "focus:ring-offset-2", "focus:ring-blue-500"
                        )
                        on:click=move |ev| {
                            ev.stop_propagation();
                            if let Some(input) = input_ref.get_untracked() {
                                input.click();
                            }
                        }
                    >
                        "Select File"
                    </button>
                </div>
                <input
                    r#type="file"
                    accept="image/*"
                    class=tw_join!("hidden")
                    on:change=on_file_change
                    on:click=on_input_click
                    node_ref=input_ref
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
fn event_target<T: JsCast>(event: &Event) -> T {
    event
        .target()
        .expect("Event should have a target")
        .dyn_into::<T>()
        .expect("Target should be of the correct type")
}
