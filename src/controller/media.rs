use crate::common::{CommonResp, Resp};
use crate::model::article::{self, Article};
use actix_web::{get, web, HttpRequest};

const RECNET_ARTICLE_NUM: i64 = 10;

// 先上传图片，之后再上传media
pub async fn save_media(media: web::Json<Media>) -> CommonResp {
    Resp::ok(media::save_media(media.into_inner())?).to_json()
}

#[get("/article/all")]
pub async fn list_all_articles() -> CommonResp {
    Resp::ok(article::list_summary_publish_articles()?).to_json()
}

#[get("/article/recent")]
pub async fn list_recent_articles() -> CommonResp {
    Resp::ok(article::list_recent_articles(RECNET_ARTICLE_NUM)?).to_json()
}

#[get("/article/get/{id}")]
pub async fn get_article(path: web::Path<(String,)>) -> CommonResp {
    let id = path.0.as_str();
    Resp::ok(article::get_publish_article(id)?).to_json()
}

// 获取文章的编辑版本
pub async fn get_edit_article(path: web::Path<(String,)>) -> CommonResp {
    let id = path.0.as_str();
    Resp::ok(article::get_edit_article(id)?).to_json()
}

// 更新相应的文章
pub async fn update_article(req: HttpRequest, article: web::Json<Article>) -> CommonResp {
    println!("haha");
    let id = req.match_info().get("id").unwrap_or("");
    if id.is_empty() {
        return Resp::err_msg("id is empty").to_json();
    }
    Resp::ok(article::update_article(id, article.into_inner())?).to_json()
}

// publish
pub async fn publish_article(req: HttpRequest, article: web::Json<Article>) -> CommonResp {
    println!("haha");
    let id = req.match_info().get("id").unwrap_or("");
    if id.is_empty() {
        return Resp::err_msg("id is empty").to_json();
    }
    Resp::ok(article::publish_article(id, article.into_inner())?).to_json()
}

// 删除对应的文章
pub async fn remove_article(path: web::Path<(String,)>) -> CommonResp {
    let id = path.0.as_str();
    if id.is_empty() {
        return Resp::err_msg("id is empty").to_json();
    }
    Resp::ok(article::remove_article(id)?).to_json()
}

pub async fn list_edit_articles() -> CommonResp {
    Resp::ok(article::list_summary_edit_articles()?).to_json()
}
