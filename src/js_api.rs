use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    /// For PoC we will display results in a dialog window.
    pub fn alert(s: &str);
}
