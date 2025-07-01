// In src/filters/core.rs

// Correct imports for photon-rs 0.3
use crate::filters::FilterType;
use photon_rs::colour_spaces::{hsv, hue_rotate_hsv};
use photon_rs::conv::{edge_detection, gaussian_blur, sharpen};
use photon_rs::effects::{adjust_brightness, adjust_contrast};
use photon_rs::monochrome::{grayscale, sepia};
use photon_rs::transform::{crop, fliph, flipv, resize, rotate};
use photon_rs::PhotonImage;

// Update the map_intensity function to handle different filter types
fn map_intensity(filter_type: FilterType, intensity: f64) -> f64 {
    match filter_type {
        // Brighten: -100 to +100 (direct mapping)
        FilterType::Brighten => intensity,
        // Contrast: 0-200% -> 0.0-2.0
        FilterType::Contrast => intensity / 100.0,
        // Hue: 0-360 degrees (direct mapping)
        FilterType::Hue => intensity,
        // Saturate: 0-200% -> 0.0-2.0
        FilterType::Saturate => intensity / 100.0,
        // Blur: 0-20px (direct mapping)
        FilterType::Blur => intensity,
        // Sharpen: 0-200% -> 0.0-2.0
        FilterType::Sharpen => intensity / 100.0,
        // Sepia: 0-100% -> 0.0-1.0
        FilterType::Sepia => intensity / 100.0,
        // Default case (shouldn't be reached for filters without intensity)
        _ => intensity,
    }
}

// Update the apply_filter function to use the mapped intensities
pub fn apply_filter(img: &mut PhotonImage, filter_type: FilterType, intensity: f64) {
    match filter_type {
        FilterType::None => {} // No filter
        FilterType::Grayscale => grayscale(img),
        FilterType::Sepia => {
            sepia_with_intensity(img, map_intensity(filter_type, intensity) as f32)
        }
        FilterType::Blur => blur_custom(img, map_intensity(filter_type, intensity) as f32),
        FilterType::Brighten => {
            // Fix type: adjust_brightness requires i16, not i32
            adjust_brightness(img, map_intensity(filter_type, intensity) as i16)
        }
        FilterType::Contrast => adjust_contrast(img, map_intensity(filter_type, intensity) as f32),
        FilterType::Hue => hue_rotate_hsv(img, map_intensity(filter_type, intensity) as f32),
        FilterType::Saturate => saturate_custom(img, map_intensity(filter_type, intensity) as f32),
        FilterType::Invert => invert_custom(img),
        FilterType::EdgeDetection => edge_detection(img),
        FilterType::Sharpen => {
            sharpen_with_intensity(img, map_intensity(filter_type, intensity) as f32)
        }
    }
}

// Add helper functions for filters with custom intensity
fn sepia_with_intensity(img: &mut PhotonImage, intensity: f32) {
    if intensity <= 0.0 {
        return;
    }

    let mut img_copy = img.clone();
    sepia(&mut img_copy);
    if intensity < 1.0 {
        // Blend with original based on intensity
        blend_with_original(img, &img_copy, intensity);
    } else {
        *img = img_copy;
    }
}

fn sharpen_with_intensity(img: &mut PhotonImage, intensity: f32) {
    if intensity <= 0.0 {
        return;
    }

    let mut img_copy = img.clone();
    sharpen(&mut img_copy);
    if intensity < 1.0 {
        // Blend with original based on intensity
        blend_with_original(img, &img_copy, intensity);
    } else {
        *img = img_copy;
    }
}

// Helper function to blend the original and filtered image
fn blend_with_original(original: &mut PhotonImage, filtered: &PhotonImage, alpha: f32) {
    let original_pixels = original.get_raw_pixels();
    let filtered_pixels = filtered.get_raw_pixels();
    let mut new_pixels = Vec::with_capacity(original_pixels.len());

    for (orig, filt) in original_pixels
        .chunks_exact(4)
        .zip(filtered_pixels.chunks_exact(4))
    {
        for i in 0..4 {
            let orig_val = orig[i] as f32;
            let filt_val = filt[i] as f32;
            let new_val = orig_val * (1.0 - alpha) + filt_val * alpha;
            new_pixels.push(new_val.round() as u8);
        }
    }

    *original = PhotonImage::new(new_pixels, original.get_width(), original.get_height());
}

// Blur implementation using photon-rs gaussian_blur
fn blur_custom(img: &mut PhotonImage, intensity: f32) {
    // Fix type: gaussian_blur requires i32, not f32
    let sigma = (intensity * 2.0).max(0.1) as i32; // Convert to i32
    gaussian_blur(img, sigma);
}

// Saturation implementation using photon-rs hsv
fn saturate_custom(img: &mut PhotonImage, intensity: f32) {
    // photon-rs hsv with "saturate" mode requires a 0.0-1.0 value
    // The intensity is already mapped from 0-200% to 0.0-2.0
    if intensity > 1.0 {
        let saturation_amount = (intensity - 1.0).clamp(0.0, 1.0);
        if saturation_amount > 0.0 {
            hsv(img, "saturate", saturation_amount);
        }
    } else if intensity < 1.0 {
        // To desaturate when intensity < 1.0
        let desaturation_amount = (1.0 - intensity).clamp(0.0, 1.0);
        if desaturation_amount > 0.0 {
            hsv(img, "desaturate", desaturation_amount);
        }
    }
}

// Custom inversion implementation (photon-rs doesn't have invert in effects)
fn invert_custom(img: &mut PhotonImage) {
    let pixels = img.get_raw_pixels();
    let mut new_pixels = Vec::with_capacity(pixels.len());

    for pixel in pixels.chunks_exact(4) {
        // Inverti RGB, mantieni alpha
        new_pixels.push(255 - pixel[0]); // R
        new_pixels.push(255 - pixel[1]); // G
        new_pixels.push(255 - pixel[2]); // B
        new_pixels.push(pixel[3]); // A (alpha)
    }

    *img = PhotonImage::new(new_pixels, img.get_width(), img.get_height());
}

// Function to get image dimensions
pub fn get_dimensions(img: &PhotonImage) -> (u32, u32) {
    (img.get_width(), img.get_height())
}

// Function to resize image
pub fn resize_image(img: &PhotonImage, width: u32, height: u32) -> PhotonImage {
    resize(
        img,
        width,
        height,
        photon_rs::transform::SamplingFilter::Nearest,
    )
}

// Function to crop image
pub fn crop_image(img: &PhotonImage, x: u32, y: u32, width: u32, height: u32) -> PhotonImage {
    crop(img, x, y, width, height)
}

// Function to rotate image
pub fn rotate_image(img: &PhotonImage, angle: f32) -> PhotonImage {
    rotate(img, angle)
}

// Function to flip horizontally
pub fn flip_horizontal(img: &PhotonImage) -> PhotonImage {
    let mut result = img.clone();
    fliph(&mut result);
    result
}

// Function to flip vertically
pub fn flip_vertical(img: &PhotonImage) -> PhotonImage {
    let mut result = img.clone();
    flipv(&mut result);
    result
}
