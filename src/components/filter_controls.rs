// src/components/filter_controls.rs

// Required imports for Leptos 0.8
use crate::filters::FilterType;
use leptos::prelude::*;

// Add this struct to define filter-specific configurations
#[derive(Clone, Copy)]
struct FilterConfig {
    min: f64,
    max: f64,
    step: f64,
    unit: &'static str,
    default: f64,
}

impl FilterConfig {
    fn for_filter(filter_type: &FilterType) -> Option<FilterConfig> {
        match filter_type {
            FilterType::Brighten => Some(FilterConfig {
                min: -100.0,
                max: 100.0,
                step: 1.0,
                unit: "%",
                default: 0.0,
            }),
            FilterType::Contrast => Some(FilterConfig {
                min: -100.0,
                max: 100.0,
                step: 1.0,
                unit: "%",
                default: 0.0,
            }),
            FilterType::Hue => Some(FilterConfig {
                min: -180.0,
                max: 180.0,
                step: 1.0,
                unit: "°",
                default: 0.0,
            }),
            FilterType::Saturate => Some(FilterConfig {
                min: -100.0,
                max: 100.0,
                step: 1.0,
                unit: "%",
                default: 0.0,
            }),
            FilterType::Blur => Some(FilterConfig {
                min: 0.0,
                max: 20.0,
                step: 0.1,
                unit: "px",
                default: 0.0,
            }),
            FilterType::Sharpen => Some(FilterConfig {
                min: 0.0,
                max: 200.0,
                step: 1.0,
                unit: "%",
                default: 0.0,
            }),
            FilterType::Sepia => Some(FilterConfig {
                min: 0.0,
                max: 100.0,
                step: 1.0,
                unit: "%",
                default: 0.0,
            }),
            _ => None,
        }
    }
}

// Constants and helper functions
const FILTER_OPTIONS: &[(FilterType, &str)] = &[
    (FilterType::None, "🚫"),
    (FilterType::Grayscale, "⚫"),
    (FilterType::Sepia, "🟤"),
    (FilterType::Blur, "🌫️"),
    (FilterType::Brighten, "☀️"),
    (FilterType::Contrast, "🔳"),
    (FilterType::Hue, "🌈"),
    (FilterType::Saturate, "🎨"),
    (FilterType::Invert, "🔄"),
    (FilterType::EdgeDetection, "📐"),
    (FilterType::Sharpen, "🔪"),
];

fn filter_name_english(filter_type: &FilterType) -> &'static str {
    match filter_type {
        FilterType::None => "None",
        FilterType::Grayscale => "Grayscale",
        FilterType::Sepia => "Sepia",
        FilterType::Blur => "Blur",
        FilterType::Brighten => "Brightness",
        FilterType::Contrast => "Contrast",
        FilterType::Hue => "Hue",
        FilterType::Saturate => "Saturation",
        FilterType::Invert => "Invert",
        FilterType::EdgeDetection => "Edge Detection",
        FilterType::Sharpen => "Sharpen",
    }
}

#[component]
pub fn FilterControls(
    #[prop(into)] on_filter_change: Callback<FilterType>,
    intensity: ReadSignal<u8>,
    #[prop(into)] on_intensity_change: Callback<u8>,
) -> impl IntoView {
    let (active_filter, set_active_filter) = signal(None::<FilterType>);

    // Function to handle filter change
    let handle_filter_change = move |filter_type: FilterType| {
        set_active_filter.set(Some(filter_type));
        on_filter_change.run(filter_type);

        if let Some(config) = FilterConfig::for_filter(&filter_type) {
            on_intensity_change.run(config.default as u8);
        }
    };

    view! {
        <div class="card bg-base-100 shadow-md mb-4">
            <div class="card-body p-4">
                <h2 class="card-title text-lg mb-4">"Image Filters"</h2>

                // Filter Selection
                <div class="form-control w-full mb-4">
                    <label class="label">
                        <span class="label-text">"Select Filter"</span>
                    </label>
                    <select
                        class="select select-bordered w-full"
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            if let Ok(filter_index) = value.parse::<usize>() {
                                if filter_index < FILTER_OPTIONS.len() {
                                    let (filter, _) = FILTER_OPTIONS[filter_index];
                                    handle_filter_change(filter);
                                }
                            }
                        }
                    >
                        {FILTER_OPTIONS
                            .iter()
                            .enumerate()
                            .map(|(index, (filter, emoji))| {
                                let filter_name = filter_name_english(filter);
                                view! {
                                    <option
                                        value=index
                                        selected=move || active_filter.get() == Some(*filter)
                                    >
                                        {format!("{emoji} {filter_name}")}
                                    </option>
                                }
                            })
                            .collect_view()}
                    </select>
                </div>

                // Intensity Control
                {move || {
                    let current_filter = active_filter.get();
                    match current_filter {
                        Some(filter) => {
                            match FilterConfig::for_filter(&filter) {
                                Some(config) => {
                                    view! {
                                        <div class="space-y-2">
                                            <label class="block text-sm font-medium text-base-content">
                                                "Intensity: "
                                                <span class="font-bold">
                                                    {move || {
                                                        format!("{:.1}{}", intensity.get() as f64, config.unit)
                                                    }}
                                                </span>
                                            </label>
                                            <input
                                                type="range"
                                                class="range range-primary w-full"
                                                min=config.min
                                                max=config.max
                                                step=config.step
                                                value=move || intensity.get() as f64
                                                on:input=move |ev| {
                                                    let value = event_target_value(&ev)
                                                        .parse::<f64>()
                                                        .unwrap_or(config.default) as u8;
                                                    on_intensity_change.run(value);
                                                }
                                            />
                                            <div class="flex justify-between text-xs text-base-content/70">
                                                <span>{format!("{}{}", config.min, config.unit)}</span>
                                                <span>{format!("{}{}", config.max, config.unit)}</span>
                                            </div>
                                        </div>
                                    }
                                        .into_any()
                                }
                                None => {
                                    view! {
                                        <div class="text-sm text-base-content/70">
                                            <p>"This filter does not require intensity adjustments."</p>
                                        </div>
                                    }
                                        .into_any()
                                }
                            }
                        }
                        None => view! { <div></div> }.into_any(),
                    }
                }}

                // Reset Button
                <div class="card-actions justify-end mt-4">
                    <button
                        class="btn btn-error btn-md normal-case w-full sm:w-40"
                        on:click=move |_| {
                            set_active_filter.set(None);
                            on_filter_change.run(FilterType::None);
                            on_intensity_change.run(0);
                        }
                    >
                        <span class="mr-2">"🔄"</span>
                        <span>"Reset Filters"</span>
                    </button>
                </div>
            </div>
        </div>
    }
}
