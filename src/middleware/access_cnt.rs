use actix_web::dev::{forward_ready, Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;
use redis::Client;
use std::pin::Pin;

pub struct AccessCnt {
    redis: Client,
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AccessCnt
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AccessCntMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let redis = self.redis.clone();
        ok(AccessCntMiddleware { service, redis })
    }
}

impl AccessCnt {
    #[allow(dead_code)]
    pub fn new(redis: Client) -> AccessCnt {
        AccessCnt { redis }
    }
}

pub struct AccessCntMiddleware<S> {
    service: S,
    redis: Client,
}

impl<S, B> Service<ServiceRequest> for AccessCntMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path();
        if !req.path().starts_with("/favicon.ico") {
            let mut con = self.redis.get_connection().unwrap();
            let incr_cnt: i32 = redis::cmd("incrby")
                .arg(path)
                .arg(1)
                .query(&mut con)
                .unwrap();
            println!("{},incr res:{}", path, incr_cnt);
        }
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            println!("Hi from response");
            Ok(res)
        })
    }
}
