文件读写
========

``` {.rust}
use std::fs::File;
use std::io::prelude::*;
// 打开文件
let mut file = match File::open(file_path) {
    Ok(f) => f,
    Err(e) => panic!("no such file {} exception:{}", file_path, e),
};
let mut str_val = String::new();
//读取文件
match file.read_to_string(&mut str_val) {
    Ok(s) => s,
    Err(e) => panic!("Error Reading file: {}", e),
};
```

toml解码
--------

``` {.rust}
// 接上篇，在文件读取完内容后，将内容decode即可，注意返回值一定要有注释，因为返回值是通过泛型传进去的
let app: AppConf = toml::from_str(&str_val).unwrap();
```
