use actix_web::{get, post, HttpRequest, Responder};

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
