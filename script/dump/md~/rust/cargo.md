install
=======

安装rustup即安装cargo:

``` {.shell}
curl https://sh.rustup.rs -sSf | sh
```

创建项目
========

``` {.shell}
cargo new hello_world
```

其生成的文件有:

| .
| ├── Cargo.toml // 项目包管理文件
| └── src // 代码文件目录
|     └── main.rs // main文件

相关命令:

-   `cargo new hello_world --bin` 创建程序，加上 `--bin`
    和不加的效果其实是一样的
-   `cargo new hello_world --lib` 创建开发库。
-   `cargo new hello_world --vcs none` 不使用版本控制工具。

编译/运行
=========

-   `cargo run` 使用该条指令进行编译并运行。在编译后会生成 `Cargo.lock`
    文件，该文件我们不需要关心，其包含了项目的包依赖信息。
-   `cargo build` 开始编译，默认使用debug模式编译，其编译结果存放在
    `target/debug` 目录下。
-   `cargo build --release` 开启编译优化进行编译，并把生成的程序放在
    `target/release` 目录下。

Cargo.toml
==========

cargo.io
是Rust社区主要的项目注册的地方，也是查找和下载项目的地方,要依赖其中的库，可以将相关配置放在Cargo.toml文件中.
如想要使用cargo.io中提供的time包，就可以在Cargo.toml中填入以下配置:

``` {.toml}
[package]  // 描述你的项目信息
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
time = "0.1.12"  //指定项目名称和版本
regex = "0.1.41" // 该项目可以在cargo.io网站上搜索到
```

如果regex的版本更新后，使用 `cargo build`
编译项目，还是会使用老版的依赖，此时需要使用 `cargo update` 来更新依赖。
在依赖存在后，就可以在代码中使用 `extern crate regex;` 来使用他了。
如果要依赖github中的库，可以通过以下方式指定:

``` {.toml}
[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git" }
```

但这样会拉最新的master分支代码，所以需要指定版本，指定方式如下:

``` {.toml}
[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git", rev = "9f35b8e" }
```

也可以指定相关的其他参数，如rev,tag,branch

如果在本地的hello~world的项目中~，又创建了一个新的项目，如命名为
`hello_utils` ,即 `cargo new hello_utils` 那么指定依赖的方式如下:

``` {.toml}
[dependencies]
hello_utils = { path = "hello_utils" }
```

其中path也可以指定其他目录，如 `../../github.com/hah`

测试
====

-   `cargo test` 运行

项目布局
========

| .
| ├── Cargo.lock
| ├── Cargo.toml
| ├── benches
| │ └── large-input.rs
| ├── examples
| │ └── simple.rs
| ├── src
| │ ├── bin
| │ │ └── another~executable~.rs
| │ ├── lib.rs
| │ └── main.rs
| └── tests
|     └── some-integration-tests.rs

Cargo.toml 和 Cargo.lock 存放在根目录 (package root)。 源代码存放在 src
目录。 默认的库文件源代码为src/lib.rs。
默认可执行文件源代码为src/main.rs。 其他可执行文件源代码可以存放在
src/bin/\*.rs。 集成测试代码放在 tests
目录(单元测试代码在所测试的代码文件中)。 示例放在examples 目录。
性能评估代码放在 benches 目录.

参考
====

详细请参考:<https://rustlang-cn.org/office/rust/cargo/reference/reference.html>
