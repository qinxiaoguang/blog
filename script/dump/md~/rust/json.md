概述
====

rust中不像golang中json转换方便，需要像应的包支持。方法如下

方式
====

引入以下包:

``` {.rust}
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0
```

注意不要使用json包，serde~json和serde是配合用的~，serde用于序列化。

使用代码:

``` {.rust}
 let res = "{\"access_token\":\"6b8afe0b8b93ea6b561d718f0963983e127f4e76\",\"token_type\":\"bearer\",\"scope\":\"\"}";
// 很简单，主要就是下列一行代码
let value: serde_json::Value = match serde_json::from_str(&res) {
     Ok(v) => v,
     Err(e) => {
         error!("json decode failed :{:?}", e);
         return None;
     }
 };
println!("{}",value["access_token"]);
println!("{}",value["access_token"].as_str().unwrap());

```

但是要注意这种方式导出的value信息，如果是string，则会将
双引号一同打印出来。需要使用 `value["xxx""].as_str()`
的方式将其转换为字符串。
