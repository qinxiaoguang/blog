use crate::common::{CommonResp, Resp};
use crate::util::file;
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
    let file_path = format!("{}{}", TMP_PATH, id);

    Resp::ok(file::get_content_or_create(file_path)).to_json()
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
    let file_path = format!("{}{}", TMP_PATH, id);
    println!(
        "id is :{:?},content is:{:?},path is :{:?}",
        id, inner_content, file_path
    );
    Resp::ok_msg(&file::save_to_file(file_path, inner_content)?).to_json()
    //Resp::ok_msg("write success").to_json()
}
