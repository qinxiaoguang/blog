use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;
use redis::Client;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct AccessCnt {
    redis: Client,
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for AccessCnt
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
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
    pub fn new(redis: Client) -> AccessCnt {
        AccessCnt { redis }
    }
}

pub struct AccessCntMiddleware<S> {
    service: S,
    redis: Client,
}

impl<S, B> Service for AccessCntMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
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
