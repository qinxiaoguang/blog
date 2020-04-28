use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConf {
    pub name: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub email: Option<String>,
    pub redis: RedisConf,
    pub server: ServerConf,
    pub mongo: MongoConf,
    pub github: GithubConf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubConf {
    pub client_id: String,
    pub client_secret: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedisConf {
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MongoConf {
    pub ip: String,
    pub port: u16,
    pub db_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConf {
    pub port: Option<u16>,
    pub page_url: Option<String>,   // 页面的url
    pub server_url: Option<String>, // 服务的url
    pub git_sync: Option<String>,   // git同步的脚本路径
}

impl AppConf {
    pub fn new(file_path: &str) -> Self {
        use std::fs::File;
        use std::io::prelude::*;
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception:{}", file_path, e),
        };
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file: {}", e),
        };
        let app: AppConf = toml::from_str(&str_val).unwrap();
        info!("appconf is :{:?}", app);
        app
    }
}
