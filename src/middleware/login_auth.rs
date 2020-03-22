use crate::common::user_helper;
use crate::util::can_match;
use actix_http::httpmessage::HttpMessage;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, http::header, Error, HttpResponse};
use futures::future::{ok, Either, Ready};
use std::task::{Context, Poll};

// 验证登录组件
pub struct LoginAuthMid {
    // 要求强制登录的url正则
    hit: Vec<String>,
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for LoginAuthMid
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoginAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let hit = self.hit.clone();
        ok(LoginAuthMiddleware {
            service: service,
            hit,
        })
    }
}

impl LoginAuthMid {
    pub fn new(hit: Vec<String>) -> LoginAuthMid {
        LoginAuthMid { hit }
    }
}

pub struct LoginAuthMiddleware<S> {
    service: S,
    hit: Vec<String>,
}

impl<S, B> Service for LoginAuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let req_path = req.path();
        let sid = match req.cookie("sessionid") {
            Some(cookie) => cookie.value().to_owned(), // 注意此处cookie内部的值是一个引用，返回的值还属于cookie,如果不使用to_owned，则会报live no long错误
            None => "".to_owned(),
        };
        println!("cookie is :{:?}", sid);
        println!("login service: your path is:{}", req_path);
        // TODO 获取cookie
        for pattern in self.hit.iter() {
            println!(
                "pattern is :{}, can_match:{},sid:{}, is_login:{}",
                pattern,
                can_match(pattern, req_path),
                &sid,
                user_helper::is_login(&sid)
            );
            // 如果可以对的上
            if can_match(pattern, req_path) && !user_helper::is_login(&sid) {
                // 执行登录过滤操作，如果登录，则pass,否则跳转登录界面
                // 需要登录
                return Either::Right(ok(req.into_response(
                    HttpResponse::Found()
                        .header(header::LOCATION, "http://localhost:8080/login.html")
                        .finish()
                        .into_body(),
                )));
            }
        }
        println!("not occured");
        // 不需要登录
        Either::Left(self.service.call(req))
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_regex() {}
}
