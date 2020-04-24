mod common;
mod config;
mod controller;
mod middleware;
mod model;
mod router;
mod util;

use actix_cors::Cors;
use actix_web::{
    get, http::header, middleware::Logger, web, App, HttpRequest, HttpServer, Responder,
};
use config::AppConf;
use lazy_static::lazy_static;
use log::info;
use log4rs;
use middleware::login_auth;
use mongodb::{options::ClientOptions, Client as MongoClient};
use redis::Client;

lazy_static! {
    pub static ref GLOBAL_CONF: AppConf = AppConf::new("conf/app.toml");
    pub static ref REDIS: Client = {
        let redis_address = format!(
            "redis://{}:{}",
            GLOBAL_CONF.redis.ip, GLOBAL_CONF.redis.port
        );
        redis::Client::open(redis_address.as_str()).unwrap()
    };
    pub static ref MONGO: MongoClient = {
        let client_options = ClientOptions::parse(&format!(
            "mongodb://{}:{}",
            GLOBAL_CONF.mongo.ip, GLOBAL_CONF.mongo.port
        ))
        .expect("Failed new mongo options");
        MongoClient::with_options(client_options).expect("Failed to initialize standalone client.")
    };
}

// 共享数据
pub struct GlobalData {
    redis_client: Client,
}

fn init_logger() {
    log4rs::init_file("conf/log4rs.yaml", Default::default()).unwrap();
    info!("env_logger initialized.");
}

#[get("/greet")]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    info!("hello world");
    let app_conf = AppConf::new("conf/app.toml");
    let port = app_conf.server.port.unwrap_or(80u16);
    let page_url = app_conf.server.page_url.clone().unwrap(); // 需要跨域的url
    let binding_address = format!("{}:{}", "0.0.0.0", port);
    let server = HttpServer::new(move || {
        let redis_address = format!("redis://{}:{}", app_conf.redis.ip, app_conf.redis.port);
        let redis_client = redis::Client::open(redis_address.as_str()).unwrap();
        let global_data = GlobalData {
            redis_client: redis_client.clone(),
        };
        App::new()
            .data(global_data)
            .wrap(login_auth::LoginAuthMid::new(
                vec!["/admin/*".to_string()],
                vec!["/admin/*".to_string()],
            ))
            //.wrap(access_cnt::AccessCnt::new(redis_client.clone()))
            // 设置response header ，解决跨域问题
            // 注意这个wrap一定要放在最后边，因为wrap的middleware执行顺序是从下往上的
            .wrap(
                Cors::new()
                    .allowed_origin(&page_url)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::ORIGIN])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(86400)
                    .finish(),
            )
            .wrap(Logger::default())
            // 设置json上限50M
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 50)) // json数据允许50MB
            .configure(router::route)
            .service(greet)
    })
    .bind(binding_address)
    .expect(&format!("can't bind to port:{:?}", port));

    server.run().await
}
