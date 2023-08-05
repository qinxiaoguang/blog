use serde_json::Value;

#[allow(dead_code)]
fn json_print(input: String) -> String {
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
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let input = r#"{"hello":"world"}"#;
        let res = json_print(input.to_string());
        println!("{:?}", res)
    }
}
