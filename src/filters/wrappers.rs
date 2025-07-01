use crate::filters::{core, FilterType};
use photon_rs::PhotonImage;
use wasm_bindgen::prelude::*;

// WebAssembly wrapper for applying filters
#[wasm_bindgen]
pub fn apply_filter_wasm(img: &mut PhotonImage, filter_type: u8, intensity: f64) {
    let filter = match filter_type {
        0 => FilterType::None,
        1 => FilterType::Grayscale,
        2 => FilterType::Sepia,
        3 => FilterType::Blur,
        4 => FilterType::Brighten,
        5 => FilterType::Contrast,
        6 => FilterType::Hue,
        7 => FilterType::Saturate,
        8 => FilterType::Invert,
        9 => FilterType::EdgeDetection,
        10 => FilterType::Sharpen,
        _ => FilterType::None,
    };

    core::apply_filter(img, filter, intensity);
}

// Get filter metadata for JavaScript
#[wasm_bindgen]
pub fn get_filter_metadata() -> JsValue {
    let filters = FilterType::all();
    let metadata: Vec<FilterMetadata> = filters
        .iter()
        .enumerate()
        .map(|(i, f)| FilterMetadata {
            id: i as u8,
            name: f.name().to_string(),
            uses_intensity: f.uses_intensity(),
        })
        .collect();

    serde_wasm_bindgen::to_value(&metadata).unwrap_or(JsValue::NULL)
}

// Filter metadata structure
#[derive(serde::Serialize)]
struct FilterMetadata {
    id: u8,
    name: String,
    uses_intensity: bool,
}

// Create a PhotonImage from raw bytes
#[wasm_bindgen]
pub fn create_photon_image(data: &[u8]) -> Result<PhotonImage, JsValue> {
    match photon_rs::native::open_image_from_bytes(data) {
        Ok(img) => Ok(img),
        Err(e) => Err(JsValue::from_str(&format!("Failed to create image: {e}"))),
    }
}

// Get image dimensions
#[wasm_bindgen]
pub fn get_image_dimensions(img: &PhotonImage) -> Box<[u32]> {
    let (width, height) = core::get_dimensions(img);
    Box::new([width, height])
}

// Resize image
#[wasm_bindgen]
pub fn resize_image_wasm(img: &PhotonImage, width: u32, height: u32) -> PhotonImage {
    core::resize_image(img, width, height)
}

// Crop image
#[wasm_bindgen]
pub fn crop_image_wasm(img: &PhotonImage, x: u32, y: u32, width: u32, height: u32) -> PhotonImage {
    core::crop_image(img, x, y, width, height)
}

// Rotate image
#[wasm_bindgen]
pub fn rotate_image_wasm(img: &PhotonImage, angle: f32) -> PhotonImage {
    core::rotate_image(img, angle)
}

// Flip image horizontally
#[wasm_bindgen]
pub fn flip_horizontal_wasm(img: &PhotonImage) -> PhotonImage {
    core::flip_horizontal(img)
}

// Flip image vertically
#[wasm_bindgen]
pub fn flip_vertical_wasm(img: &PhotonImage) -> PhotonImage {
    core::flip_vertical(img)
}
