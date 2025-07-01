pub mod core;
pub mod wrappers;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterType {
    None,
    Grayscale,
    Sepia,
    Blur,
    Brighten,
    Contrast,
    Hue,
    Saturate,
    Invert,
    EdgeDetection,
    Sharpen,
}

impl FilterType {
    pub fn name(&self) -> &'static str {
        match self {
            FilterType::None => "None",
            FilterType::Grayscale => "Grayscale",
            FilterType::Sepia => "Sepia",
            FilterType::Blur => "Blur",
            FilterType::Brighten => "Brighten",
            FilterType::Contrast => "Contrast",
            FilterType::Hue => "Hue Rotate",
            FilterType::Saturate => "Saturate",
            FilterType::Invert => "Invert",
            FilterType::EdgeDetection => "Edge Detection",
            FilterType::Sharpen => "Sharpen",
        }
    }

    pub fn uses_intensity(&self) -> bool {
        !matches!(
            self,
            FilterType::None
                | FilterType::Grayscale
                | FilterType::Invert
                | FilterType::EdgeDetection
        )
    }

    pub fn all() -> Vec<FilterType> {
        vec![
            FilterType::None,
            FilterType::Grayscale,
            FilterType::Sepia,
            FilterType::Blur,
            FilterType::Brighten,
            FilterType::Contrast,
            FilterType::Hue,
            FilterType::Saturate,
            FilterType::Invert,
            FilterType::EdgeDetection,
            FilterType::Sharpen,
        ]
    }
}

impl fmt::Display for FilterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub fn apply_filter(img: &mut photon_rs::PhotonImage, filter_type: FilterType, intensity: f64) {
    core::apply_filter(img, filter_type, intensity);
}
