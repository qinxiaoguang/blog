use serde_json::{Result, Value};
use wasm_bindgen::prelude::*;

// Reverse a string coming from JS
#[wasm_bindgen]
pub fn hello_world() -> String {
    String::from("hello world")
}

// Reverse a string coming from JS
#[wasm_bindgen]
pub fn reverse(s: String) -> String {
    s.chars().rev().collect::<String>()
}

#[wasm_bindgen]
pub fn json_print(input: String) -> String {
    let v: Value = match serde_json::from_str(&input) {
        Ok(v) => v,
        Err(e) => return format!("err:{:?}", e),
    };
    match serde_json::to_string_pretty(&v) {
        Ok(r) => r,
        Err(e) => return format!("err:{:?}", e),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
