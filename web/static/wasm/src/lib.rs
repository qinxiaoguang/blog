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

#[wasm_bindgen]
pub fn unicode_to(input: String) -> String {
    let s: String = serde_json::from_str(&format!("\"{}\"", input)).unwrap();
    s
}

#[wasm_bindgen]
pub fn to_unicode(input: String) -> String {
    //let res: Vec<String> =
    input
        .encode_utf16()
        .map(|b| format!("\\u{:x}", b))
        .collect::<Vec<String>>()
        .join("")
    //res.join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {}

    #[test]
    fn unicode_parse_test() {
        let input = "\\u54c8\\u54c8\\u54c8";
        let output = "哈哈哈";
        assert_eq!(output, unicode_parse(String::from(input)));
        assert_eq!(to_unicode(String::from("哈哈哈")), input);
    }
}
