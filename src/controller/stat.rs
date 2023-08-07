/**
 * file:tools.rs
 * des: 统计相关接口
 * time: 2019-11-26
 * author: qxg
 */
use crate::{common::constants::KNOCK_COUNT, GlobalData};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use redis::Commands;

#[allow(dead_code)]
pub fn test(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

// TODO 获取访问当前网站的访问个数
#[get("/stat/cnt")]
pub async fn get_access_cnt(data: web::Data<GlobalData>, _req: HttpRequest) -> impl Responder {
    let mut con = data.redis_client.get_connection().unwrap();
    let count: Result<i32, _> = con.get(KNOCK_COUNT);
    match count {
        Ok(c) => HttpResponse::Ok().json(c),
        Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("{:?}", e)),
    }
}
