区别和联系
==========

Fn、FnMut、FnOnce都是trait,看下源码就大致明白其中的区别和联系:

``` {.rust}
pub trait FnOnce<Args> {
    /// The returned type after the call operator is used.
    #[stable(feature = "fn_once_output", since = "1.12.0")]
    type Output;

    /// Performs the call operation.
    #[unstable(feature = "fn_traits", issue = "29625")]
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

pub trait FnMut<Args>: FnOnce<Args> {
    /// Performs the call operation.
    #[unstable(feature = "fn_traits", issue = "29625")]
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait Fn<Args>: FnMut<Args> {
    /// Performs the call operation.
    #[unstable(feature = "fn_traits", issue = "29625")]
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}
```

继承关系: Fn \<- FnMut \<-
FnOnce,也即Fn可以被当作FnMut及FnOnce,但FnMut不能被当作Fn来用。
再看其中的方法Fn是捕获的 `&self`
,所以在调用一次Fn的方法后，没有消耗掉其生命周期，所以还能继续使用，同样的FnMut不仅能继续重复调用，还能修改其环境，而FnOnce则只能调用一次,且不能修改其环境。

但是FnMut
是怎么修改其环境的呢，这不就是一个函数吗，函数修改自己的什么环境呢。其实不是函数修改自己的环境，而是函数内部的代码修改外部的环境，比如外部有一个变量Vec,那么使用FnMut的话，该代码里就可以改变外部的Vec变量，如代码举例:

``` {.rust}
fn main() {
    let mut v: Vec<String> = Vec::new();
    test(move || v.push("haha".to_string()))
}

fn test<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}
```

如果闭包里的变量使用到了mut类型，那么该闭包就是mut类型。如上述demo,其中的
`v.push()` 使用到的v就是mut类型，所以此时的闭包 `||v.push()`
就是FnMut类型。

而对于move而言，可以将闭包理解为一个结构体，当使用move修饰的时候，会将对应的变量move到对应的结构体中，类似创建了一个包含变量的Fn/FnMut/FnOnce的结构体。如可以将
`move || v.push("haha".to_string())` 看做 `FnMut{v}`
因为变量v被move了，所以后边不能再被使用。

fn和Fn
======

fn是函数指针,是一个类型，不是trait，他实现了Fn/FnMut/FnOnce三个trait,所以参数是Fn/FnMut/FnOnce都可以将函数传递过去,如:

``` {.rust}
fn p(){
    println!("haha");
}


fn test(f:fn()){
    f();
}


fn test<F:Fn()>(f:F){
    f();
}

// 以下函数均生效
//    test2(p);
//    test2(||{println!("heiei")});
//    test(p); // 因为函数实现了Fn，所以可以将p传递给test<F:Fn()>(f:F)
//    test(||{println!("heiei")});

```
