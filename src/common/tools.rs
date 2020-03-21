/**
 * file:tools.rs
 * des: some block code blocked in one place.
 * its different from util,it should be used by some global variable.
 */
use actix_http::http::{self, StatusCode};
use actix_http::Response;
use actix_web::HttpResponse;

// 统一redirect
pub fn redirect(url: &str) -> Response {
    HttpResponse::build(StatusCode::TEMPORARY_REDIRECT)
        .header(http::header::LOCATION, url)
        .finish()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_serde_json() {
        let res = "{\"access_token\":\"6b8afe0b8b93ea6b561d718f0963983e127f4e76\",\"token_type\":\"bearer\",\"scope\":\"\"}";
        let value: serde_json::Value = serde_json::from_str(&res).unwrap();
        let access_token = value["access_token"].as_str().unwrap();
        println!("{}", access_token);
        let res = format!("token {}", access_token);
        println!("{:?}", res);
    }
    #[test]
    fn test_string() {
        let token = &"haha";
        let res = format!("token {}", token);
        println!("{}", res);
    }
}
