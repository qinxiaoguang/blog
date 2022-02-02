use chrono::prelude::*;
use serde_json::Value;
use urlencoding::{decode, encode};
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

#[wasm_bindgen]
pub fn get_timestamp() -> i64 {
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let china_timezone = FixedOffset::east(8 * 3600);
    utc_time.with_timezone(&china_timezone).timestamp()
}

#[wasm_bindgen]
pub fn get_time_from_unix(ts: i64) -> String {
    let t = chrono::NaiveDateTime::from_timestamp(ts, 0);
    let china_timezone = FixedOffset::east(8 * 3600);
    let res = DateTime::<Utc>::from_utc(t, Utc);
    let res = res.with_timezone(&china_timezone);
    res.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[wasm_bindgen]
pub fn urldecode(input: String) -> String {
    match decode(&input) {
        Ok(res) => {
            format!("{}", res)
        }
        Err(e) => {
            format!("err:{}", e)
        }
    }
}

#[wasm_bindgen]
pub fn urlencode(input: String) -> String {
    encode(&input).to_string()
}

#[wasm_bindgen]
pub fn int_to_ip(input: u32) -> String {
    let mut res = String::from("");
    let mask = 255;
    for i in 0..4 {
        let tmp = (input >> i * 8) & mask;
        res = String::from(".") + &tmp.to_string() + &res;
    }
    String::from(&res[1..])
}

#[wasm_bindgen]
pub fn ip_to_int(input: String) -> u32 {
    let items: Vec<&str> = input.split(".").collect();
    if items.len() != 4 {
        return 0;
    }
    let mut res = 0;

    items.iter().rev().enumerate().for_each(|(idx, &item)| {
        let tmp: u32 = item.parse().unwrap();
        res += tmp << (8 * idx);
    });
    res
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
        assert_eq!(output, unicode_to(String::from(input)));
        assert_eq!(to_unicode(String::from("哈哈哈")), input);
    }

    #[test]
    fn timestamp() {
        println!("{}", get_timestamp());
        println!("{}", get_time_from_unix(1642443514))
    }

    #[test]
    fn urlencode_test() {
        println!("{}", urlencode(String::from("http://haha")));
        println!("{}", urldecode(String::from("http%3A%2F%2Fhahaa")));
    }

    #[test]
    fn ip_test() {
        assert_eq!(int_to_ip(123123), String::from("0.1.224.243"));
        assert_eq!(int_to_ip(1), String::from("0.0.0.1"));
        assert_eq!(int_to_ip(4294967295), String::from("255.255.255.255"));

        assert_eq!(ip_to_int(String::from("0.1.224.243")), 123123u32);
        assert_eq!(ip_to_int(String::from("0.0.0.1")), 1);
        assert_eq!(ip_to_int(String::from("255.255.255.255")), 4294967295);
    }
}
