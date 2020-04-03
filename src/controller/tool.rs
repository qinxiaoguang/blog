use crate::common::{CommonResp, Resp};
use actix_web::{get, post, web, HttpRequest};
use serde::{Deserialize, Serialize, Serializer};
use std::fs::File;
use std::io::prelude::*;

const TMP_PATH: &str = "tmpfile/";

#[get("/tool/tmpedit/{id}")]
pub async fn get_content(path: web::Path<(String,)>) -> CommonResp {
    //TODO 将文件读取封装为util
    let id = path.0.as_str();
    if id.is_empty() {
        return Resp::err_msg("id cant be empty;").to_json();
    }
    let file_path = format!("{}/{}", TMP_PATH, id);

    let mut file = match File::open(&file_path) {
        Err(_) =>
        // 打开失败，则创建
        {
            File::create(&file_path)?
        }
        Ok(f) => f,
    };

    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Resp::ok(content).to_json()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TmpContent {
    id: String,
    content: String,
}

#[post("/tool/tmpedit")]
pub async fn save_content(content: web::Json<TmpContent>) -> CommonResp {
    let id = content.id.clone();
    if id.is_empty() {
        return Resp::err_msg("id cant be empty;").to_json();
    }
    let inner_content = content.content.clone();
    let file_path = format!("{}/{}", TMP_PATH, id);
    let mut file = match File::open(&file_path) {
        Err(_) =>
        // 打开失败，则创建
        {
            File::create(&file_path)?
        }
        Ok(f) => f,
    };
    file.write_all(inner_content.as_bytes())?;

    Resp::ok_msg("write success").to_json()
}
