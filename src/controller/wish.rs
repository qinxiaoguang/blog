use crate::common::{CommonResp, Resp};
use crate::model::wish::{self, Wish};
use actix_web::{get, web};

// 先上传图片，之后再上传media
#[allow(dead_code)]
pub async fn save_wish(wish: web::Json<Wish>) -> CommonResp {
    let wish = wish.into_inner();
    Resp::ok(wish::save_wish(wish)?).to_json()
}

#[get("/wish/random")]
pub async fn random_wish() -> CommonResp {
    Resp::ok(wish::get_random_wish()?).to_json()
}

#[get("/wish/all")]
pub async fn all_wish() -> CommonResp {
    Resp::ok(wish::list_all_wish()?).to_json()
}
