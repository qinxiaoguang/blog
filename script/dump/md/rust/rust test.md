cfg
===

单例
----

``` {.rust}
// 直接使用cfg进行测试，使用cargo test即可进行测试
#[cfg(test)]
mod tests {
    // 如果要测试本代码中的私有函数 ，需要使用use super::*;来引入私有函数
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

集成
----

需要在src的同级目录中创建tests文件，并创建相关文件，使用 `use xxx::xx`
来引入src中的代码，集成测试不需要使用cfg，如下:

``` {.rust}
use adder; // src目录中的包

#[test]  // 不需要cfg
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

使用 `cargo test` 即可进行测试。
