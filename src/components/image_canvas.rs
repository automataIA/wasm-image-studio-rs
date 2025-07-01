// src/components/image_canvas.rs
use crate::filters::FilterType;
use leptos::prelude::*;
use std::panic;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

#[allow(non_snake_case)]
#[component]
pub fn ImageCanvas(
    image_src: Signal<Option<String>>,
    active_filter: ReadSignal<Option<FilterType>>,
    filter_intensity: ReadSignal<u8>,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Canvas>>,
    #[prop(optional, into)] on_processing_complete: Option<Callback<()>>,
) -> impl IntoView {
    let canvas_ref = node_ref.unwrap_or_else(|| NodeRef::<leptos::html::Canvas>::new());
    let (is_processing, set_is_processing) = signal(false);
    let (canvas_width, set_canvas_width) = signal(800);
    let (canvas_height, set_canvas_height) = signal(600);
    let (canvas_ready, set_canvas_ready) = signal(false);

    let original_image_data = RwSignal::<Option<Vec<u8>>>::new(None);

    panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Effect to verify canvas is ready - IMPROVED
    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            // Wait for canvas to be actually in the DOM and rendered
            let check_canvas_ready = {
                let canvas = canvas.clone();
                Closure::wrap(Box::new(move || {
                    // Verify multiple conditions to ensure canvas is ready
                    if canvas.parent_node().is_some()
                        && canvas.offset_width() > 0
                        && canvas.offset_height() > 0
                    {
                        // Test context for Chromium
                        match canvas.get_context("2d") {
                            Ok(Some(_)) => {
                                set_canvas_ready.set(true);
                                web_sys::console::log_1(
                                    &"✅ Canvas is ready and context available".into(),
                                );
                            }
                            Ok(None) => {
                                web_sys::console::log_1(
                                    &"⏳ Canvas context not yet available, retrying...".into(),
                                );
                                // Retry after another frame
                                request_animation_frame_with_delay(
                                    move || {
                                        set_canvas_ready.set(true);
                                    },
                                    16,
                                );
                            }
                            Err(e) => {
                                web_sys::console::error_1(
                                    &format!("❌ Canvas context error: {e:?}").into(),
                                );
                            }
                        }
                    } else {
                        web_sys::console::log_1(
                            &"⏳ Canvas not yet fully mounted, waiting...".into(),
                        );
                    }
                }) as Box<dyn FnMut()>)
            };

            request_animation_frame_closure(check_canvas_ready);
        }
    });

    // Effect for image loading - IMPROVED
    Effect::new(move |_| {
        if canvas_ready.get() {
            if let Some(src) = image_src.get() {
                web_sys::console::log_1(
                    &format!(
                        "🖼️ Image source updated: {}... (length: {} bytes)",
                        &src[..src.len().min(50)],
                        src.len()
                    )
                    .into(),
                );

                set_is_processing.set(true);
                web_sys::console::log_1(&"⏳ Starting image loading process".into());

                let img = match HtmlImageElement::new() {
                    Ok(img) => img,
                    Err(e) => {
                        web_sys::console::error_1(
                            &format!("Failed to create image element: {e:?}").into(),
                        );
                        set_is_processing.set(false);
                        return;
                    }
                };

                let canvas_ref_clone = canvas_ref;
                let original_image_data_clone = original_image_data;
                let set_is_processing_clone = set_is_processing;
                let set_canvas_width_clone = set_canvas_width;
                let set_canvas_height_clone = set_canvas_height;

                use std::rc::Rc;
                let img_rc = Rc::new(img);
                let img_clone = img_rc.clone();

                let onload_callback = Closure::wrap(Box::new(move || {
                    web_sys::console::log_1(&"✅ Image onload event triggered".into());

                    if let Some(canvas) = canvas_ref_clone.get_untracked() {
                        let natural_width = img_clone.natural_width();
                        let natural_height = img_clone.natural_height();

                        web_sys::console::log_1(
                            &format!(
                                "📐 Image loaded - Natural dimensions: {natural_width}x{natural_height}"
                            )
                            .into(),
                        );

                        let (width, height) = calculate_dimensions(&img_clone);
                        web_sys::console::log_1(
                            &format!("📏 Calculated display dimensions: {width}x{height}").into(),
                        );

                        set_canvas_width_clone.set(width);
                        set_canvas_height_clone.set(height);

                        canvas.set_width(width as u32);
                        canvas.set_height(height as u32);

                        let img_for_draw = img_clone.clone();
                        let canvas_for_draw = canvas.clone();

                        // Wait two frames to ensure canvas is updated
                        request_animation_frame_with_delay(
                            move || {
                                match get_canvas_context_safe(&canvas_for_draw) {
                                    Ok(ctx) => {
                                        web_sys::console::log_1(
                                            &"🎨 Got canvas context successfully".into(),
                                        );

                                        ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

                                        if let Err(e) = ctx.draw_image_with_html_image_element(
                                            &img_for_draw,
                                            0.0,
                                            0.0,
                                        ) {
                                            web_sys::console::error_1(
                                                &format!("❌ Failed to draw image: {e:?}").into(),
                                            );
                                        } else {
                                            web_sys::console::log_1(
                                                &"✅ Image successfully drawn to canvas".into(),
                                            );

                                            match ctx.get_image_data(
                                                0.0,
                                                0.0,
                                                width as f64,
                                                height as f64,
                                            ) {
                                                Ok(image_data) => {
                                                    let data = image_data.data();
                                                    let data_vec: Vec<u8> = data.to_vec();
                                                    web_sys::console::log_1(
                                                        &format!(
                                                            "💾 Extracted image data: {} bytes",
                                                            data_vec.len()
                                                        )
                                                        .into(),
                                                    );
                                                    original_image_data_clone.set(Some(data_vec));
                                                }
                                                Err(e) => {
                                                    web_sys::console::error_1(
                                                        &format!(
                                                        "❌ Failed to extract image data: {e:?}"
                                                    )
                                                        .into(),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        web_sys::console::error_1(
                                            &format!("❌ Failed to get canvas 2D context: {e}")
                                                .into(),
                                        );
                                    }
                                }
                                set_is_processing_clone.set(false);
                            },
                            32,
                        ); // Wait 2 frames (32ms)
                    } else {
                        web_sys::console::error_1(&"❌ Canvas reference is None".into());
                        set_is_processing_clone.set(false);
                    }
                }) as Box<dyn FnMut()>);

                img_rc.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
                img_rc.set_src(&src);
                onload_callback.forget();
            }
        }
    });

    // Effect for filters
    Effect::new(move |_| {
        let filter = active_filter.get();
        let intensity = filter_intensity.get();

        if canvas_ready.get() {
            if let Some(filter_type) = filter {
                web_sys::console::log_1(
                    &format!("Filter changed to {filter_type:?} with intensity {intensity}").into(),
                );

                if let Some(canvas) = canvas_ref.get_untracked() {
                    if let Some(image_data) = original_image_data.get() {
                        set_is_processing.set(true);

                        request_animation_frame(move || {
                            apply_filter(
                                &canvas,
                                &image_data,
                                filter_type,
                                intensity,
                                set_is_processing,
                                on_processing_complete,
                            );
                        });
                    }
                }
            }
        }
    });

    view! {
        <div class="image-canvas-container">
            <canvas
                node_ref=canvas_ref
                width=move || canvas_width().to_string()
                height=move || canvas_height().to_string()
                class="image-canvas"
                style="display: block; border: 1px solid #ccc;"
            ></canvas>
            <div class="loading-overlay" class:hidden=move || !is_processing()>
                <div class="spinner"></div>
                <p>Processing...</p>
            </div>
        </div>
    }
}

fn calculate_dimensions(img: &HtmlImageElement) -> (i32, i32) {
    let max_width = 1400;
    let max_height = 900;

    let width = img.natural_width() as i32;
    let height = img.natural_height() as i32;

    web_sys::console::log_1(&format!("Original dimensions: {width}x{height}").into());

    if width <= 0 || height <= 0 {
        web_sys::console::error_1(&"Invalid image dimensions".into());
        return (max_width, max_height);
    }

    if width > max_width || height > max_height {
        let width_ratio = max_width as f64 / width as f64;
        let height_ratio = max_height as f64 / height as f64;
        let ratio = width_ratio.min(height_ratio);

        let new_width = (width as f64 * ratio) as i32;
        let new_height = (height as f64 * ratio) as i32;

        web_sys::console::log_1(
            &format!("Scaled to: {new_width}x{new_height} (ratio: {ratio:.2})").into(),
        );

        (new_width, new_height)
    } else {
        web_sys::console::log_1(&"No scaling needed".into());
        (width, height)
    }
}

// IMPROVED FUNCTION to safely handle context
#[allow(non_snake_case)]
fn get_canvas_context_safe(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, String> {
    if canvas.width() == 0 || canvas.height() == 0 {
        return Err("Canvas has zero dimensions".to_string());
    }

    // Verify that canvas is actually in the DOM
    if canvas.parent_node().is_none() {
        return Err("Canvas is not in DOM".to_string());
    }

    // Try to get context with retry for Chromium
    // Try to get context with retry for Chromium
    for attempt in 0..3 {
        match canvas.get_context("2d") {
            Ok(Some(ctx)) => match ctx.dyn_into::<CanvasRenderingContext2d>() {
                Ok(context) => return Ok(context),
                Err(e) => {
                    web_sys::console::warn_1(
                        &format!("Attempt {}: Failed to cast context: {:?}", attempt + 1, e).into(),
                    );
                }
            },
            Ok(None) => {
                web_sys::console::warn_1(
                    &format!("Attempt {}: Context is None", attempt + 1).into(),
                );
            }
            Err(e) => {
                web_sys::console::warn_1(
                    &format!("Attempt {}: Failed to get context: {:?}", attempt + 1, e).into(),
                );
            }
        }

        // Wait a bit before next attempt
        if attempt < 2 {
            // In a real environment you should use a timeout, here we use an empty loop
            for _ in 0..1000 { /* small delay */ }
        }
    }

    Err("Failed to get 2D context after multiple attempts".to_string())
}

fn apply_filter(
    canvas: &HtmlCanvasElement,
    original_data: &[u8],
    filter_type: FilterType,
    intensity: u8,
    set_is_processing: WriteSignal<bool>,
    on_complete: Option<Callback<()>>,
) {
    match get_canvas_context_safe(canvas) {
        Ok(ctx) => {
            let width = canvas.width();
            let height = canvas.height();

            let mut photon_img = photon_rs::PhotonImage::new(original_data.to_vec(), width, height);
            crate::filters::core::apply_filter(&mut photon_img, filter_type, intensity as f64);

            let processed_data = photon_img.get_raw_pixels();
            let clamped_data = wasm_bindgen::Clamped(&processed_data[..]);

            match web_sys::ImageData::new_with_u8_clamped_array_and_sh(clamped_data, width, height)
            {
                Ok(image_data) => {
                    if let Err(e) = ctx.put_image_data(&image_data, 0.0, 0.0) {
                        web_sys::console::error_1(
                            &format!("Failed to put image data: {e:?}").into(),
                        );
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to create ImageData: {e:?}").into());
                }
            }

            set_is_processing.set(false);
            if let Some(cb) = on_complete {
                cb.run(());
            }
        }
        Err(e) => {
            web_sys::console::error_1(&format!("Canvas context error: {e}").into());
            set_is_processing.set(false);
            if let Some(cb) = on_complete {
                cb.run(());
            }
        }
    }
}

fn request_animation_frame<F>(f: F)
where
    F: FnOnce() + 'static,
{
    let window = web_sys::window().expect("No global window exists");
    let closure = Closure::once(f);
    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("Failed to request animation frame");
    closure.forget();
}

// NEW FUNCTIONS for better timing management
fn request_animation_frame_closure(closure: Closure<dyn FnMut()>) {
    let window = web_sys::window().expect("No global window exists");
    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("Failed to request animation frame");
    closure.forget();
}

fn request_animation_frame_with_delay<F>(f: F, delay_ms: i32)
where
    F: FnOnce() + 'static,
{
    let window = web_sys::window().expect("No global window exists");
    let closure = Closure::once(f);
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            delay_ms,
        )
        .expect("Failed to set timeout");
    closure.forget();
}
