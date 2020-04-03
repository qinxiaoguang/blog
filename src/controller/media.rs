use crate::common::{CommonResp, Resp};
use crate::model::media::{self, Media};
use actix_web::{get, web};

const RECNET_ARTICLE_NUM: i64 = 2;

// 先上传图片，之后再上传media
pub async fn save_media(media: web::Json<Media>) -> CommonResp {
    let media = media.into_inner();
    if media.url.is_none() {
        return Resp::err_msg("url cant be null").to_json();
    }
    Resp::ok(media::save_media(media)?).to_json()
}

#[get("/media/all")]
pub async fn list_all_media() -> CommonResp {
    Resp::ok(media::list_all_media()?).to_json()
}

#[get("/media/recent")]
pub async fn list_recent_media() -> CommonResp {
    Resp::ok(media::list_recent_media(RECNET_ARTICLE_NUM)?).to_json()
}

#[get("/media/img/recent")]
pub async fn list_recent_img() -> CommonResp {
    Resp::ok(media::list_recent_img(RECNET_ARTICLE_NUM)?).to_json()
}

// 删除对应的文章
pub async fn remove_media(path: web::Path<(String,)>) -> CommonResp {
    let id = path.0.as_str();
    if id.is_empty() {
        return Resp::err_msg("id is empty").to_json();
    }
    Resp::ok(media::remove_media(id)?).to_json()
}
