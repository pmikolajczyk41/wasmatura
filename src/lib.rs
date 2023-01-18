mod js_api;
mod types;
mod universities;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{js_api::alert, types::MaturaResults, universities::process_matura_results};

#[wasm_bindgen]
/// The only method in our API: converts serialized `MaturaResults` into serialized
/// `RecruitmentResults`.
pub fn process(data: &str) {
    let matura_results: MaturaResults =
        serde_json::de::from_str(data).expect("Failed to deserialize matura results");

    let recruitment_results = process_matura_results(&matura_results);
    let recruitment_results = serde_json::ser::to_string(&recruitment_results)
        .expect("Failed to serialize recruitment results");

    alert(&format!("Recruitment results: {recruitment_results}"));
}
