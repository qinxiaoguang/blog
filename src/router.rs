use crate::controller::{article, oauth, ping, stat, upload};

use actix_web::web;
// 配置
pub fn route(sc: &mut web::ServiceConfig) {
    sc.service(stat::get_access_cnt)
        .service(article::list_all_articles)
        .service(article::list_recent_articles)
        .service(article::get_article)
        .service(oauth::oauth_callback)
        .service(ping::ping)
        .service(web::scope("/admin/upload").route("/pic", web::post().to(upload::upload_pic)))
        .service(
            web::scope("/admin/article") // 使用scope来将某一类route聚合,说白了就是前缀
                .route("", web::post().to(article::save_article))
                .route("/get/{id}", web::get().to(article::get_edit_article))
                .route("{id}", web::put().to(article::update_article))
                .route("/publish/{id}", web::put().to(article::publish_article))
                .route("{id}", web::delete().to(article::remove_article))
                .route("/edit/list", web::get().to(article::list_edit_articles)),
        );
}
