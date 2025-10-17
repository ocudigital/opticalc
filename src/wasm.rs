use wasm_bindgen::prelude::*;

use crate::{
    convert_power, convert_rx, crossed_cylinders, induced_prism, minimum_blank_size,
    oblique_meridian, recommended_blank_size, simulate_lensmeter_reading, transpose, CombinedPrism,
    Decentration, Eye, SpheroCyl,
};

#[wasm_bindgen(js_name = convertPower)]
pub fn convert_power_wasm(measured_power_diopters: f64, n_assumed: f64, n_actual: f64) -> f64 {
    convert_power(measured_power_diopters, n_assumed, n_actual)
}

#[wasm_bindgen(js_name = convertRx)]
pub fn convert_rx_wasm(measured: SpheroCyl, n_assumed: f64, n_actual: f64) -> SpheroCyl {
    convert_rx(measured, n_assumed, n_actual)
}

#[wasm_bindgen(js_name = simulateLensmeterReading)]
pub fn simulate_lensmeter_reading_wasm(
    true_rx: SpheroCyl,
    n_assumed: f64,
    n_actual: f64,
) -> SpheroCyl {
    simulate_lensmeter_reading(true_rx, n_assumed, n_actual)
}

#[wasm_bindgen(js_name = crossedCylinders)]
pub fn crossed_cylinders_wasm(lens1: SpheroCyl, lens2: SpheroCyl) -> SpheroCyl {
    crossed_cylinders(lens1, lens2)
}

#[wasm_bindgen(js_name = transpose)]
pub fn transpose_wasm(lens: SpheroCyl) -> SpheroCyl {
    transpose(lens)
}

#[wasm_bindgen(js_name = obliqueMeridian)]
pub fn oblique_meridian_wasm(lens: SpheroCyl, axis_deg: f64) -> f64 {
    oblique_meridian(lens, axis_deg)
}

#[wasm_bindgen(js_name = minimumBlankSize)]
pub fn minimum_blank_size_wasm(
    effective_diameter_mm: f64,
    eyesize_mm: f64,
    bridge_mm: f64,
    ipd_mm: f64,
) -> f64 {
    minimum_blank_size(effective_diameter_mm, eyesize_mm, bridge_mm, ipd_mm)
}

#[wasm_bindgen(js_name = recommendedBlankSize)]
pub fn recommended_blank_size_wasm(
    effective_diameter_mm: f64,
    eyesize_mm: f64,
    bridge_mm: f64,
    ipd_mm: f64,
) -> f64 {
    recommended_blank_size(effective_diameter_mm, eyesize_mm, bridge_mm, ipd_mm)
}

#[wasm_bindgen(js_name = inducedPrism)]
pub fn induced_prism_wasm(eye: Eye, lens: SpheroCyl, dec: Decentration) -> CombinedPrism {
    induced_prism(eye, lens, dec)
}
