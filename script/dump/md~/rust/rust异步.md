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
    block_on(test());
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

在rust中，aysnc,await实际上就是语法糖，async可以修饰方法，代码块等等，其将对应的方法和代码块修饰为Future, 而await其实就是等待Future运行结束。

相关库
======

# async_std
async_std相对的就是std标准库，async_std提供了std相对的异步库，如`fs, task::sleep`等等。
rust中同步函数是不能调用异步函数的，如果想要同步函数调用异步函数，就需要async_std中或是tokio中的一些方法来支持，如 block_on方法。

rust中的main函数就是同步函数，所以可以通过block_on来调用异步方法，如:
``` rust
use async_std::task;

fn main() {
    task::block_on(async {
        println!("Hello, world!");
    })
}
```
而异步函数中是可以调用同步方法的，那为什么不把main变为异步方法呢，当然也是可以的，通过注解的方式，不过要开启actix_std的相关feature,如:

Cargo.toml:
```toml
async-std = {version = "1.5.0", features = ["attributes"]}
```

main.rs:
```rust
#[async_std::main]
async fn main() -> Result<()>{}
```
而 golang里的main函数() 其实就是一个异步(gorotine)函数

废话不多说，举个例子，相关解释都在注释里了:
```rust
use rayon::prelude::*;
use anyhow::Result;
use async_std::task;

#[async_std::main]
// main是同步函数，如果想在同步函数中运行异步函数，一个方法是使用block_on
// 还有一个方法就是使用注释的方式，将main函数变为异步函数
// golang里的main函数() 其实就是一个异步函数
async fn main() -> Result<()>{
    use std::fs::File;
    use std::io::Read;
    use walkdir::WalkDir;

    for entry in WalkDir::new("/Users/qinxiaoguang01/proj/rust/kvs").follow_links(true)
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|e| !e.path().is_dir())
        .filter(|e|!e.path().to_str().unwrap().to_owned().contains("target")){
            println!("{:?}",entry.path().display());
            // 线程池的个数一般是与num cpus的个数相关
            println!("num cpus:{:?}",num_cpus::get());
            task::spawn(async move {
                // 如果只是使用async {} .await,那么这个异步方法只在一个线程里运行
                // tokio::spawn 相当于golang里的go func,会将任务放在线程池中运行，但并不等于，golang里的gorotine是绿色线程
                // 后面换成了task::spawn, 还是用着async_std顺手
                // 但是如果发起的async任务里面，都是阻塞的方法
                // 那其实就相当于发起了一个线程,该阻塞方法会阻塞一个线程，导致该线程无法继续运行其他async代码
                // 所以更推荐的是，将阻塞的方法，换成相应的异步执行方式
                // 如 File::open, 使用async_std::fs::File::open().await
                let path = entry.path();
                let mut content = String::new();
                File::open(path).unwrap().read_to_string(&mut content).unwrap();
                println!("{:?}",content);
                // 同样的道理，sleep也要换成非阻塞的
                // async_std::task::sleep().awit
                std::thread::sleep(std::time::Duration::from_secs(10));
            });
        }
    Ok(())
}
```
async_std和tokio基本上一样，用哪个都可以，也可以混用，不过可能会导致线程双倍的快乐。

在异步代码里使用阻塞的代码，会阻塞当前线程，那当前线程下的所有的async任务都不会再被执行，而是全部等待，这个时候的协程就和线程没有任何区别，所以一定要注意，最好的方式是在异步代码里使用异步的函数或库，比如跟文件相关的使用`async_std::fs`等。

还有一些不宜察觉的阻塞，比如println!(),在async代码中使用println!会发生阻塞，解决方案是使用 `async_std`中的宏`println`来代替。

如果有些阻塞代码实在没辙，可以将这些阻塞代码，放到task::spawn_blocking()中去运行，或者是tokio::spawn_blocking中，他们会自动将这些代码放到专门的线程池中运行，而不会阻塞当前线程。

但是如果通过spawn_blocking来运行，需要将相关参数进行clone,因为spawn_blocking是创建一个新的线程，新的线程不会捕获变量所有权，只能通过clone来拿到相关参数，这样的话就会有些低效。解决的方法就是通过tokio::block_in_place, 该方法是同步的方法,唯一特殊的是，他会将当前的async代码发送给新的线程来运行。

另外[该文章](https://async.rs/blog/stop-worrying-about-blocking-the-new-async-std-runtime/)声称不必担心在异步里写阻塞代码，因为会自动检测阻塞代码，并将其做特殊处理。

怎么自动检测的呢? 方法就是runtime会计算当前代码的阻塞时间，达到某个临界值的时候，便认为其发生了阻塞，如果阻塞代码执行的很快，就不会发生新线程的创建，也就不会造成任何额外的消耗。使用新版的async_std就不需要担心async代码中使用阻塞代码。基本上效果是和golang里的go func一样

但是实际测试时，还是会有阻塞的情况出现，所以最好的方式还是尽量在异步代码里使用异步的包，比如async_std::fs等。

新的async_std包建议你，将阻塞代码放到新的任务里，如 `spawn(async move{ your code }).await`

另外，如果要测试golang和rust的并发性能比较，一定要加上`--release`,曾经的我比较性能，使用debug方式来跑rust代码，性能死活比不了golang,后来加上了`--release` 那性能真是没得说。
  
如果在`task::spawn()`里嵌套使用`task::spawn`，类比`go func(){go func(){}}`,你可能会有一些困扰，在`task::spawn({})`里运行的代码，如 
```rust
    task::spawn(async{
        println!("hoho");
        task::spawn(async{println!("heihi");});
        println!("lala");
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
```
你会发现无论怎么运行，他都是先运行第一层的代码，之后才会去运行嵌套的`task::spawn(async{println!("heihi");});`代码，也就是说，在`task::spawn`内部，无论你再怎么`task::spawn`，他都不会生成一个新的线程，而是生成了一个任务链，使用当前的线程来运行这个任务链，如果任务链有阻塞，那么就会一直等待。

rayon
-----
将iter变为parallel-iter,如 `a.iter()` -> `a.par_iter()`

如
```rust
use rayon::prelude::*;
fn main(){
	let arr = vec![1,2,3];
	let max = arr.par_iter().cloned().max();
}
```

参考
====

<https://www.jianshu.com/p/b5e347b3a17c>
