概述
====

rust中的http库有actix~http~,http,hyper等，但使用起来都不太顺手，偶然间看到reqwest库，这个库挺符合心意，遂拿来编码。
在网络编程方面，如果你不知道某个功能使用哪个库，可以在这个地方来找相应的库:<https://lib.rs/web-programming>

以下内容，你可以在文档: <https://docs.rs/reqwest/0.9.22/reqwest/>
中找到，但为了方便，稍作记录。

get方法
=======

``` {.rust}
//获取body
client::get(url).text()
// 发送带header的请求，请参考下边的post方法
```

post
====

创建携带header的client

``` {.rust}
// 创建client
let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()
    .expect("cant build");

// post传参
let params = [
    ("client_id", &GLOBAL_CONF.github.client_id),
    ("client_secret", &GLOBAL_CONF.github.client_secret),
    ("code", &code.to_string()),
];
// headers
let mut headers = header::HeaderMap::new();
headers.insert(
    header::ACCEPT,
    header::HeaderValue::from_static("application/json"),
);
// url
let url = "https://github.com/login/oauth/access_token".to_string();
// 发送请求并处理结果
let res = match client.post(&url).form(&params).send() {
    Ok(mut res) => {
        res.text().unwrap()
    }
    Err(e) => {
        return None;
    }
};
```

使用默认client，每次发送请求携带相应的header:

``` {.rust}
let client = reqwest::Client::new();
//post
client.post(&url).header(header::AUTHORIZATION,format!("token {}", access_token)).send()
// get
client.get(&url).header(header::AUTHORIZATION,format!("token {}", access_token)).send()
```
