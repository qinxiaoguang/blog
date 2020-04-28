概览
====

rust的异步由几个关键字组成async/await/Future等，但是看了官方文档，真是像吞粪一样，太晦涩难懂，于是参考其他语言的async,await才把这个关键搞懂。
rust的协程和go有很大的出入，其实现方式和java中的，js中的都非常类似，首先要弄懂其协程，就要先搞懂aysnc,await的含义。
简单来说，async是修饰一个函数，使其成为异步函数，如

``` {.rust}
use std::{thread,time};
async fn test(){
    println!("{}","我后执行");
}

fn main(){
    test();
    println!("{}","我先执行"); // 先执行此处
    let one_second = time::Duration::from_millis(1000);
    thread::sleep(one_second);
}
```

而await表示当前执行的命令如果造成阻塞，就转去执行其他协程。

在js中，理解async/await的前提是理解generator及yield.
不详述，用语言简单描述下，generator函数中会使用yield，其中yield关键字的作用是暂停执行点，并回传yield后的表达式的值。其中generator在创建后并不会立马执行，而是在执行.next方法后，会执行到第一处yield处，并返回，注意加yield和不加yield程序都是可以运行的，区别就是一个可以暂停，一个不可以。
如

``` {.javascript org-language="js"}
function * gen(){
    yield '1';
    yield '2';
    return '3';
}

var g = gen(); //创建的时候不会执行。
g.next(); // 返回 {value:'1', done: false}
g.next(); // 返回 {value:'2', done: false}
g.next(); // 返回 {value:'3', done: true}
g.next(); // 返回 {value:'undefined', done: true},后边返回都一样。
```

基础库
======

rayon
-----

将iter变为parallel~iter~,如 `a.iter()` -\> `a.par_iter()`

参考
====

<https://www.jianshu.com/p/b5e347b3a17c>
