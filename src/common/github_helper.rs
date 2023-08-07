// github helper
use crate::*;
use reqwest::header;

use super::Result;

// 通过github登录回调的code获取access_token,并通过access_token接获取name
pub async fn get_name_with_code(code: &str) -> Result<String> {
    match get_access_token(code).await {
        Ok(access_token) => get_user_name(&access_token).await,
        Err(e) => Err(e),
    }
}

// 通过github回调的code获取access_token
pub async fn get_access_token(code: &str) -> Result<String> {
    let params = [
        ("client_id", &GLOBAL_CONF.github.client_id),
        ("client_secret", &GLOBAL_CONF.github.client_secret),
        ("code", &code.to_string()),
    ];
    let url = "https://github.com/login/oauth/access_token".to_string();
    let client = reqwest::Client::new();
    let value = client
        .post(&url)
        .header(header::ACCEPT, "application/json")
        .form(&params)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("get res is :{:?}", value);
    let res =
        value["access_token"]
            .as_str()
            .map(|x| x.to_owned())
            .ok_or(super::BizError::CommonError {
                field: String::from("get access_token data error"),
            });
    println!("get result is:{:?}", res);
    return res;
}

// 通过access_token获取login_name,注意当前只获取login_name,扩展方法后期再考虑
pub async fn get_user_name(access_token: &str) -> Result<String> {
    let url = "https://api.github.com/user".to_string();
    //let url = "http://localhost:8888".to_string();
    let client = reqwest::Client::new();
    println!("start to get user");
    let value = client
        .get(&url)
        .header(header::AUTHORIZATION, format!("Bearer  {}", access_token))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("get user is :{:?}", value);
    value["login"]
        .as_str()
        .map(|x| x.to_owned())
        .ok_or(super::BizError::CommonError {
            field: String::from("get login data error"),
        })
}

mod test {
    #[test]
    fn test_value() {
        let mut m = serde_json::Map::new();
        m.insert(
            String::from("access_token"),
            serde_json::Value::String("haha".to_string()),
        );
        let value = serde_json::Value::Object(m);
        println!("get value is:{:?}", value);
        let g = value["access_token"]
            .as_str()
            .map(|x| x.to_owned())
            .ok_or(0);
        println!("get res is:{:?}", g);
    }
}
