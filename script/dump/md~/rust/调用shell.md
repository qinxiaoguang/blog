## 方法

通过使用`std::process::Command`即可实现，如下代码:

```rust
use std::process::Command;

Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start");
```

如果要获取其中的输出，则可以这样:
```rust
use std::process::Command;
let output = Command::new("/bin/cat")
                     .arg("file.txt")
                     .output()
                     .expect("failed to execute process");
```