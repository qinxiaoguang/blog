clap是一个cli相关的包，其全名是:`Command Line Arguments Parse`.看官方的介绍，使用一般是这样的:
```rust
#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Same as previous examples...
}
```
编译时提示 ，load_yaml找不到。

考虑了下，因为rust在2018版本后，基本已经废弃了extern关键字，所以将代码调整成如下:
```rust
use clap::{load_yaml, App};
fn main() {
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();
}

```
编译时提示 `maybe a missing crate clap`, 嘿我特码的纳闷了，我不是引用了clap吗，怎么就引用失败了。

所以这个时候去官网找文档，发现其需要使用引用一个feature,这个feature就是yaml,所以cargo.toml中需要这么写:
```rust
clap = { version = "2.33.0", features= ["yaml"] }
```
原因就是load_yaml使用到了yaml(yaml-rust)的这个包，如果不使用这个feature的话，load_yaml是用不了的。到此编译成功。

那么feature是什么，作用是什么呢?

# features
此feature非future,不要搞混，他的作用如下：
- 条件编译的可选项: cfg
- 可选的依赖包，用来增强程序，但不是必须的，比如上边如果想要使用load_yaml这种增强功能，就需要yaml依赖包。

一般一个crate包会在Cargo.toml中来声明自己包的features可能的值，比如clap就在他自己的项目中使用了以下feature:
```rust
[features]
default     = ["suggestions", "color", "derive", "std", "cargo"]
std         = [] # support for no_std in a backwards-compatible way
suggestions = ["strsim"]
color       = ["atty", "termcolor"]
wrap_help   = ["term_size", "textwrap/term_size"]
derive      = ["clap_derive", "lazy_static"]
yaml        = ["yaml-rust"]
cargo       = [] # Disable if you're not using Cargo, enables Cargo-env-var-dependent macros
unstable    = ["clap_derive/unstable"] # for building with unstable clap features (doesn't require nightly Rust) (currently none)
debug       = ["clap_derive/debug"] # Enables debug messages
doc         = ["yaml"] # All the features which add to documentation
```
如果你在用clap的时候，需要其中的某个feature组的时候，配置到自己项目的Cargo.toml中即可。比如`clap = { version = "2.33.0", features= ["yaml"] }`其实就是使用到了`yaml        = ["yaml-rust"]`


# 使用方式
配置`cli.yml`来配置命令行可能用到的参数，如:
```yaml
name: myapp
version: "1.0"
author: Kevin K. <kbknapp@gmail.com>
about: Does awesome things
args:
  - config:
      short: c
      long: config
      value_name: FILE
      help: Sets a custom config file
      takes_value: true
  - INPUT:
      help: Sets the input file to use
      required: true
      index: 1
  - verbose:
      short: v
      multiple: true
      help: Sets the level of verbosity
subcommands:
  - test:
      about: controls testing features
      version: "1.3"
      author: Someone E. <someone_else@other.com>
      args:
        - debug:
            short: d
            help: print debug information

```
代码里可以通过value_of来获取配置项的值，然后再对其值再做一些操作等等。
```rust
use clap::{load_yaml, App};
fn main() {
    let yaml = load_yaml!("../cli.yml");
    let m = App::from_yaml(yaml).get_matches();
    // 获取config参数的值
    match m.value_of("config") {
        Some(v) => println!("{:?}", v),
        None => {}
    }
}
```