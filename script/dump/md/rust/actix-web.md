使用
====

使用cargo创建一个项目，并在Cargo.toml中添加以下依赖即可:

``` {.toml}
[dependencies]
actix-web = "0.7"
```

在main.rs中编写以下内容:

``` {.rust}
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, get,guard};
use std::sync::Mutex;

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

// 每个请求处理器(Request Handler)都是一个函数
// 该函数的入参可以有0个或多个参数，返回值是一个实现了Responder的对象。如HttpResponse
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

// 使用宏来生成路径,通过这种方式需要在App中使用service方法来注册
#[get("/hello/world2")]
fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world2")
}

// scope的使用
fn scope() -> impl Responder {
    HttpResponse::Ok().body("scope test")
}

// 共享数据
struct AppData {
    app_name : String,
}

// 共享数据
fn get_data(data : web::Data<AppData>) ->String{
    let app_name = &data.app_name;
    format!("Hello {}!", &app_name)
}

// 配置
fn config(cfg: &mut web::ServiceConfig){
     cfg.service(
         web::resource("/qxg/qxg")
             .route(web::get().to(|| HttpResponse::Ok().body("app")))
             .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

// 可变共享数据
struct Counter {
    cnt : Mutex<i32>,
}

// 能够统计请求该route/方法的数量
fn count(data : web::Data<Counter>) -> String{
    let mut cnt = data.cnt.lock().unwrap();
    *cnt += 1;
    format!("Request number: {}",cnt)
}

fn main() {
    // 可变的共享data
    let counter = web::Data::new(Counter {
        cnt: Mutex::new(0),
    });
    HttpServer::new(move || {
        // 创建一个App实例，并在其中注册router
        // 将该App实例传递给Server即可进行绑定以及接收请求
        App::new()
            // 共享的data，后边的服务都可获取该data
            .data(AppData {
                app_name: String::from("name"),
            })
            .register_data(counter.clone()) // 可变的共享data需要注册
            .configure(config)  //将部分路由配置通过方法传入
            .route("/route/cnt",web::get().to(count))
            .route("/get/date",web::get().to(get_data))
            .route("/", web::get().to(index))
            .route("/test/", web::get().to(greet))
            .route("/{name}", web::get().to(greet)) // 可获取路由中的名字
            .service(index2)  //通过宏来设置的路由
            .service(
                web::scope("/scope") // 使用scope来将某一类route聚合,说白了就是前缀
                .guard(guard::Header("Host", "www.rust-lang.org")) // 使用guard来过滤请求
                .route("/test",web::get().to(scope))
                .route("/test2",web::get().to(scope))
            )
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
```

HttpServer
==========

``` {.rust}
use actix_web::{web, App, HttpResponse, HttpServer};
use std::sync::mpsc;
use std::thread;

pub fn main() {
    // 类比golang的channel就对了，别多想
    // 其中tx是发送器，rx是接收器
    let (tx, rx) = mpsc::channel();
    // 开启一个新的线程，来执行http server
    thread::spawn(move || {
        // actix_rt是actix的runtime包
        let sys = actix_rt::System::new("http-server");

        // HttpServer是一个Actor,其会返回一个addr的Actor模型
        // 可以给该addr发送pause,resume,stop等消息
        let addr = HttpServer::new(|| {
            App::new().route("/", web::get().to(|| HttpResponse::Ok()))
        })
        .bind("127.0.0.1:8088")
        .unwrap()
        .shutdown_timeout(60) // <- Set shutdown timeout to 60 seconds
        .start();

        // 将addr发送到channel中
        let _ = tx.send(addr);
        let _ = sys.run();
    });

    // main线程接收到addr
    let addr = rx.recv().unwrap();
    // 向此addr发送pause消息，进行暂停工作
    let _ = addr
        .pause();
    // 向此addr发送resume消息，进行恢复工作
    let _ = addr
        .resume();
    // 向此addr发送stop消息，进行停止工作
    let _ = addr
        .stop(true);
}
```

actix-web默认会开启多个线程来执行程序，其数量默认为cpu核数，而可以通过worker()方法来指定个数。

``` {.rust}
use actix_web::{web, App, HttpResponse, HttpServer};

pub fn main() {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .workers(4); // <- Start 4 workers，开启4个线程。
}
```

Request Handlers
================

请求处理器，接收0个或多个参数(impl FromRequest)，并返回 `impl Responder`
,actix-web默认为一些基础类型实现了Responder,如 \'static str及String.
如果要返回自定义类型，需要实现Responder,其中该trait中需要实现respond~to方法~。如:

``` {.rust}
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// 实现Responder
impl Responder for MyObj {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    // 实现respond_to
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

// Request handler
fn index() -> impl Responder {
    MyObj { name: "user" }
}
```

Request参数
===========

可以通过直接传入Path及Json来获取请求信息:

``` {.rust}
//获取path信息及json信息
fn index(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}
```

可以通过extract来提取相关path及json信息

``` {.rust}
// 传入HttpRequest
fn extract(req: HttpRequest) -> impl Responder {
    // 通过extract来提取
    let params = web::Path::<(String, String)>::extract(&req).unwrap();
    // 通过extract来提取
    let info = web::Json::<MyInfo>::extract(&req)
        .wait()
        .expect("Err with reading json.");

    format!("{} {} {} {}", params.0, params.1, info.username, info.id)
}
```

其中path提取url中的参数:如访问的形式是/users/{userid}/{name}，那么就可以通过web:Path\<(u32,String)\>来获取其中的userid和name.
如:

``` {.rust}
fn index(info: web::Path<(u32, String)>) -> Result<String> {
    Ok(format!("Welcome {}, userid {}!", info.1, info.0))
}

HttpServer::new(|| {
    App::new().route(
        "/users/{userid}/{friend}", // <- define path parameters
        web::get().to(index),
    )
})
.bind("127.0.0.1:8088")
.unwrap()
.run()
.unwrap();
```

而web::Path也可以传入自定义类型，不过该类型需要实现Deserialize,如:

``` {.rust}
#[derive(Deserialize)]
struct Info {
    userid: u32,
    friend: String,
}
fn index(info : web::Path<Info>)
// 其他省略
```

也可以通过HttpRequest的get或者query方法来获取path中的信息，如:

``` {.rust}
fn index(req: HttpRequest) -> Result<String> {
    // get
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    // query
    let userid: i32 = req.match_info().query("userid").parse().unwrap();

    Ok(format!("Welcome {}, userid {}!", name, userid))
}
```

通过web::Query来获取url中的filed信息:如/user?username=xxx

``` {.rust}
#[derive(Deserialize)]
struct Info {
    username: String,
}

// this handler get called only if the request's query contains `username` field
fn index(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}
```

可以通过web::Json来获取Json数据.

``` {.rust}
#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body
fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}
```

通过web::Form来获取表单信息。

``` {.rust}
#[derive(Deserialize)]
struct FormData {
    username: String,
}

/// extract form data using serde
/// this handler gets called only if the content type is *x-www-form-urlencoded*
/// and the content of the request could be deserialized to a `FormData` struct
fn index(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.username))
}
```

Error
=====

可以像golang那样返回error，返回的error,actix-web会自动进行渲染。

``` {.rust}
use actix_web::{error, Result};
use failure::Fail;
use log::debug; // 可以使用该函数来打印日志。

#[derive(Fail, Debug)]
#[fail(display = "my error")]
pub struct MyError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

fn index() -> Result<&'static str, MyError> {
    let err = MyError { name: "test error" };
    debug!("{}", err);   // 使用debug宏来打印错误日志
    Err(err)
}

pub fn main() {
    use actix_web::{middleware::Logger, web, App, HttpServer};

    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
```

其中ResponseError的定义如下,其只有一个函数，且有默认实现。

``` {.rust}
pub trait ResponseError: Fail {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
```

读取配置
========

..
