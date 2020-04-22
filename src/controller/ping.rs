use crate::common::{constants, Resp};
use crate::REDIS as rds;
use actix_web::{get, post, HttpRequest, Responder};
use log::info;

#[allow(dead_code)]
#[get("/admin/ping")]
pub async fn ping(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

#[allow(dead_code)]
#[post("/admin/pong")]
pub async fn pong(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

#[allow(dead_code)]
#[get("/knock")]
pub async fn knock(_: HttpRequest) -> impl Responder {
    let mut con = rds.get_connection().unwrap();
    let incr_cnt: i32 = redis::cmd("incrby")
        .arg(constants::KNOCK_COUNT)
        .arg(1)
        .query(&mut con)
        .unwrap();
    info!("knock result:{}", incr_cnt);
    Resp::ok(incr_cnt).to_json()
}
