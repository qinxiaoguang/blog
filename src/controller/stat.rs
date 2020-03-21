/**
 * file:tools.rs
 * des: 统计相关接口
 * time: 2019-11-26
 * author: qxg
 */
use crate::GlobalData;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

#[allow(dead_code)]
pub fn test(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

// TODO 获取访问当前网站的访问个数
#[get("/stat/cnt")]
pub async fn get_access_cnt(data: web::Data<GlobalData>, _req: HttpRequest) -> impl Responder {
    let mut con = data.redis_client.get_connection().unwrap();
    // todo 使用keys效率会比较低，后续可以改为scan
    let keys: Vec<String> = redis::cmd("keys").arg("/*").query(&mut con).unwrap();
    if keys.is_empty() {
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(String::from("redis connect error"));
    }
    let res: Vec<(String, i32)> = keys
        .into_iter()
        .map(|k| {
            let res: i32 = redis::cmd("get").arg(&k).query(&mut con).unwrap();
            (k, res)
        })
        .collect();
    HttpResponse::Ok().json(res)
}
