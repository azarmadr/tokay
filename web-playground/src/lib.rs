use wasm_bindgen::prelude::*;

use tokay::eval;

#[wasm_bindgen]
pub fn run(code: &str, input: &str) -> String {
    match eval(code, input, None) {
        Ok(value) => value.repr(),
        Err(error) => error.to_string(),
    }
}

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
