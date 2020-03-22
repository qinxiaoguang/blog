// github helper
use crate::*;
use log::*;
use reqwest::header;

// 通过github登录回调的code获取access_token,并通过access_token接获取name
pub fn get_name_with_code(code: &str) -> Option<String> {
    get_access_token(code).and_then(|access_token| get_user_name(&access_token))
}

// 通过github回调的code获取access_token
pub fn get_access_token(code: &str) -> Option<String> {
    let params = [
        ("client_id", &GLOBAL_CONF.github.client_id),
        ("client_secret", &GLOBAL_CONF.github.client_secret),
        ("code", &code.to_string()),
    ];
    let url = "https://github.com/login/oauth/access_token".to_string();
    let client = reqwest::Client::new();
    let res = match client
        .post(&url)
        .header(header::ACCEPT, "application/json")
        .form(&params)
        .send()
    {
        Ok(mut res) => {
            let res = res.text().unwrap(); // res.text()会消耗res,后续再调用将为空
            info!(
                "req access_token success, url is: {}, res is:{:?}",
                url, &res
            );
            res
        }
        Err(e) => {
            error!(
                "req access_token failed, url is :{}, error is :{:?}",
                url, e
            );
            return None;
        }
    };
    let value: serde_json::Value = match serde_json::from_str(&res) {
        Ok(v) => v,
        Err(e) => {
            error!("json decode failed :{:?}", e);
            return None;
        }
    };
    value["access_token"].as_str().map(|x| x.to_owned())
}

// 通过access_token获取login_name,注意当前只获取login_name,扩展方法后期再考虑
pub fn get_user_name(access_token: &str) -> Option<String> {
    let url = "https://api.github.com/user".to_string();
    //let url = "http://localhost:8888".to_string();
    let client = reqwest::Client::new();
    let res = match client
        .get(&url)
        .header(header::AUTHORIZATION, format!("token {}", access_token))
        .send()
    {
        Ok(mut res) => {
            let res = res.text().unwrap(); // res.text()会消耗res,后续再调用将为空
            info!("get user success, url is: {}, res is:{:?}", url, &res);
            res
        }
        Err(e) => {
            error!("req user failed, url is :{}, error is :{:?}", url, e);
            return None;
        }
    };
    let value: serde_json::Value = match serde_json::from_str(&res) {
        Ok(v) => v,
        Err(e) => {
            error!("json decode failed :{:?}", e);
            return None;
        }
    };
    value["login"].as_str().map(|x| x.to_owned())
}
