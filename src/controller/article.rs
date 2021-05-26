use crate::common::{CommonResp, Resp};
use crate::model::article::{self, Article};
use actix_web::{get, web, HttpRequest};

const RECNET_ARTICLE_NUM: i64 = 10;
const ARTICLE_PAGE_SIZE: i64 = 10;

pub async fn save_article(article: web::Json<Article>) -> CommonResp {
    Resp::ok(article::save_article(article.into_inner())?).to_json()
}

#[get("/article/all")]
pub async fn list_all_articles() -> CommonResp {
    Resp::ok(article::list_summary_publish_articles()?).to_json()
}

#[get("/article/page/{page_num}")]
pub async fn list_page_articles(path: web::Path<(i64,)>) -> CommonResp {
    let page_num = path.0;
    Resp::ok(article::list_page_articles(ARTICLE_PAGE_SIZE,page_num)?).to_json()
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
    let id = req.match_info().get("id").unwrap_or("");
    if id.is_empty() {
        return Resp::err_msg("id is empty").to_json();
    }
    Resp::ok(article::update_article(id, article.into_inner())?).to_json()
}

// publish
pub async fn publish_article(req: HttpRequest, article: web::Json<Article>) -> CommonResp {
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

#[get("/article/dump")]
pub async fn dump() -> CommonResp {
    article::dump();
    Resp::ok("task done").to_json()
}
