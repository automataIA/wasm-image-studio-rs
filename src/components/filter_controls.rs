// In src/components/filter_controls.rs

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
                min: 0.0,
                max: 200.0,
                step: 1.0,
                unit: "%",
                default: 100.0,
            }),
            FilterType::Hue => Some(FilterConfig {
                min: 0.0,
                max: 360.0,
                step: 1.0,
                unit: "°",
                default: 0.0,
            }),
            FilterType::Saturate => Some(FilterConfig {
                min: 0.0,
                max: 200.0,
                step: 1.0,
                unit: "%",
                default: 100.0,
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

fn filter_name_ita(filter_type: &FilterType) -> &'static str {
    match filter_type {
        FilterType::None => "Nessuno",
        FilterType::Grayscale => "Scala di Grigi",
        FilterType::Sepia => "Seppia",
        FilterType::Blur => "Sfocatura",
        FilterType::Brighten => "Luminosità",
        FilterType::Contrast => "Contrasto",
        FilterType::Hue => "Tonalità",
        FilterType::Saturate => "Saturazione",
        FilterType::Invert => "Inverti",
        FilterType::EdgeDetection => "Rilevamento Bordi",
        FilterType::Sharpen => "Nitidezza",
    }
}

#[component]
pub fn FilterControls(
    #[prop(into)] on_filter_change: Callback<FilterType>,
    intensity: ReadSignal<u8>,
    #[prop(into)] on_intensity_change: Callback<u8>, // Add #[prop(into)]
) -> impl IntoView {
    let (active_filter, set_active_filter) = signal(None::<FilterType>);

    // Function to handle filter change - FIX HERE
    let handle_filter_change = move |filter_type: FilterType| {
        set_active_filter.set(Some(filter_type));
        on_filter_change.run(filter_type); // ✅ Usa .run()

        if let Some(config) = FilterConfig::for_filter(&filter_type) {
            on_intensity_change.run(config.default as u8); // ✅ Usa .run()
        }
    };

    view! {
        <div class="space-y-4 p-4 border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 shadow-sm">
            <h3 class="text-lg font-medium text-gray-900 dark:text-white">Filtri Immagine</h3>
            <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    "Seleziona Filtro"
                </label>
                <select
                    class="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-md
                           bg-white dark:bg-gray-700 text-gray-900 dark:text-white
                           focus:ring-blue-500 focus:border-blue-500"
                    on:change=move |ev| {
                        let idx = event_target_value(&ev).parse::<usize>().unwrap_or(0);
                        if let Some((filter_type, _)) = FILTER_OPTIONS.get(idx) {
                            handle_filter_change(*filter_type);
                        }
                    }
                >
                    {FILTER_OPTIONS
                        .iter()
                        .enumerate()
                        .map(|(idx, (filter_type, emoji))| {
                            let is_selected = move || { active_filter.get() == Some(*filter_type) };
                            view! {
                                <option value=idx selected=is_selected>
                                    {format!("{} {}", emoji, filter_name_ita(filter_type))}
                                </option>
                            }
                        })
                        .collect::<Vec<_>>()}
                </select>
            </div>

            {move || {
                let current_filter = active_filter.get();
                match current_filter {
                    Some(filter) => {
                        match FilterConfig::for_filter(&filter) {
                            Some(config) => {
                                view! {
                                    <div class="space-y-2">
                                        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
                                            "Intensità: "
                                            <span class="font-bold">
                                                {move || {
                                                    format!("{:.1}{}", intensity.get() as f64, config.unit)
                                                }}
                                            </span>
                                        </label>
                                        <input
                                            type="range"
                                            class="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-lg appearance-none cursor-pointer"
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
                                        <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400">
                                            <span>{format!("{}{}", config.min, config.unit)}</span>
                                            <span>{format!("{}{}", config.max, config.unit)}</span>
                                        </div>
                                    </div>
                                }
                                    .into_any()
                            }
                            None => {
                                view! {
                                    <div class="text-sm text-gray-600 dark:text-gray-400">
                                        <p>
                                            "This filter does not require intensity adjustments."
                                        </p>
                                    </div>
                                }
                                    .into_any()
                            }
                        }
                    }
                    None => view! { <div></div> }.into_any(),
                }
            }}

            <button
                class="w-full px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600 dark:bg-red-600 dark:hover:bg-red-700 transition-colors"
                on:click=move |_| {
                    set_active_filter.set(Some(FilterType::None));
                    on_filter_change.run(FilterType::None);
                    on_intensity_change.run(0);
                }
            >
                "🔄 Reimposta Filtri"
            </button>
        </div>
    }
}
