//! WebAssembly bindings for the compiler

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_source(source: &str) -> Result<JsValue, JsValue> {
    // TODO: Implement parsing
    Ok(JsValue::NULL)
}
