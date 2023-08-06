use crate::common::{user_helper, Resp, NEEDLOGIN_ERROR};
use crate::util::can_match;
use actix_web::body::EitherBody;
use actix_web::dev::{forward_ready, Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use futures::future::{ok, LocalBoxFuture, Ready};
use log::info;

// 验证登录组件
pub struct LoginAuthMid {
    // 要求强制登录的url正则
    hit: Vec<String>,
    // 管理页路由
    admin: Vec<String>,
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for LoginAuthMid
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = LoginAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let hit = self.hit.clone();
        let admin = self.admin.clone();
        ok(LoginAuthMiddleware {
            service: service,
            hit,
            admin,
        })
    }
}

impl LoginAuthMid {
    pub fn new(hit: Vec<String>, admin: Vec<String>) -> LoginAuthMid {
        LoginAuthMid { hit, admin }
    }
}

pub struct LoginAuthMiddleware<S> {
    service: S,
    hit: Vec<String>,
    admin: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for LoginAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    //type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let req_path = req.path();
        let sid = match req.cookie("sessionid") {
            Some(cookie) => cookie.value().to_owned(), // 注意此处cookie内部的值是一个引用，返回的值还属于cookie,如果不使用to_owned，则会报live no long错误
            None => "".to_owned(),
        };
        info!("cookie is :{:?}", sid);
        for pattern in self.hit.iter() {
            info!(
                "pattern is :{}, can_match:{},sid:{}, is_login:{}",
                pattern,
                can_match(pattern, req_path),
                &sid,
                user_helper::is_login(&sid)
            );
            if can_match(pattern, req_path) && !user_helper::is_login(&sid) {
                // 执行登录过滤操作，如果登录，则pass,否则跳转登录界面
                // 需要登录
                let (request, _pl) = req.into_parts();
                return Box::pin(async {
                    Ok(ServiceResponse::new(
                        request,
                        HttpResponse::Ok()
                            .json(Resp::err(NEEDLOGIN_ERROR, "need login").into_json())
                            .map_into_right_body(),
                    ))
                });
            }
        }
        for pattern in self.admin.iter() {
            if can_match(pattern, req_path) && !user_helper::is_owner(&sid) {
                // 若当前路由为管理页面，并且登录的用户不是本人，则拒绝通过
                let (request, _pl) = req.into_parts();
                return Box::pin(async {
                    Ok(ServiceResponse::new(
                        request,
                        HttpResponse::MethodNotAllowed()
                            .finish()
                            .map_into_right_body(),
                    ))
                });
            }
        }
        info!("no need to login");
        // 不需要登录
        let res = self.service.call(req);
        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_regex() {}
}
