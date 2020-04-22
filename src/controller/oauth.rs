use crate::{common::*, GlobalData, GLOBAL_CONF};
use actix_http::http::{header, StatusCode};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use log::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    code: String,
}

#[get("/oauth/callback")]
pub async fn oauth_callback(
    _data: web::Data<GlobalData>,
    _req: HttpRequest,
    info: web::Query<Info>,
) -> impl Responder {
    let code = info.code.to_owned();
    if code.is_empty() {
        error!("login info is empty");
        // 注意这个地方的重定向的返回值不能是impl Responder
        // expected type `impl actix_web::responder::Responder`
        //  found type `actix_http::response::Response`
        return redirect(&GLOBAL_CONF.server.page_url.clone().unwrap());
    }
    match github_helper::get_name_with_code(&code) {
        Some(username) => {
            info!("user : {} login success", &username);
            // 登录成功，生成session-id
            let sid = session_helper::generate_session_id(&username).unwrap();
            // 将session-id保存在redis中，setcookie, 并redirect.
            redis_helper::store_to_redis(&sid, &code, Some(7 * 24 * 60 * 60)).unwrap();
            HttpResponse::build(StatusCode::FOUND)
                .header(
                    header::SET_COOKIE,
                    format!("sessionid={};expires={};path=/", sid, 7),
                )
                .header(
                    header::LOCATION,
                    format!("{}", GLOBAL_CONF.server.page_url.clone().unwrap()),
                )
                .finish()
        }
        None => {
            // 未登录，返回失败
            // 重定向404
            redirect(&(GLOBAL_CONF.server.page_url.clone().unwrap() + "/404"))
        }
    }
    // 将当前名字放入redis中，并生成对应的session-id，及cookie,设置过期时间，
    // println!("{:?}", req);
    // Resp::ok("ha").to_json();
}
#[cfg(test)]
mod tests {
    #[test]
    pub fn test_date() {
        use chrono::prelude::*;
        use chrono::Duration;
        let utc: DateTime<Utc> = Utc::now();

        let tomo = utc + Duration::days(1);
        println!("{:?}", tomo.to_string());

        println!("{:?}", Local::now().to_rfc2822());
    }
}
