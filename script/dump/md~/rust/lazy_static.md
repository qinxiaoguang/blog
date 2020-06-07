概述
====

rust中不能像go或者其他语言中一样生命一个全局变量，来让所有的代码使用，比如redis,conf这些内容，都可以通过全局变量的形式存在。好在rust中有一个包可以提供这种功能，那就是lazy~static~!

使用
====

使用方式就是通过lazy~static宏~，首先在Cargo.toml中导入包:

``` {.rust}
lazy_static = "1.4.0"
```

接着在main.rs文件中，通过lazy~static~!宏编写全局变量:

``` {.rust}
lazy_static! {
    //全局配置文件
    pub static ref GLOBAL_CONF: AppConf = AppConf::new("conf/app.toml");
    // 全局redis client
    pub static ref REDIS: Client = {
        let redis_address = format!(
            "redis://{}:{}",
            GLOBAL_CONF.redis.ip, GLOBAL_CONF.redis.port
        );
        let redis_client = redis::Client::open(redis_address.as_str()).unwrap();
        redis_client
    };
    // 全局mong client
    pub static ref MONGO: MongoClient = {
        MongoClient::connect(&GLOBAL_CONF.mongo.ip, GLOBAL_CONF.mongo.port)
            .expect("Failed to initialize standalone client.")
    };
}
```

在其他文件中使用的时候，通过引入: `use crate::*` 即可进行使用。
