use crate::controller::{article, oauth, ping, stat};

use actix_web::web;
// 配置
pub fn route(sc: &mut web::ServiceConfig) {
    sc.service(stat::get_access_cnt)
        .service(article::list_all_articles)
        .service(article::list_recent_articles)
        .service(article::get_article)
        .service(oauth::oauth_callback)
        .service(ping::ping)
        .service(ping::pong)
        .service(
            web::scope("/admin/article") // 使用scope来将某一类route聚合,说白了就是前缀
                .route("", web::post().to(article::save_article))
                .route("{id}", web::post().to(article::update_article))
                .route("{id}", web::delete().to(article::remove_article)),
        );
}
