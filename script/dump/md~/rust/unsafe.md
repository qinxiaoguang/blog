概述
====

unsafe是不安全的意思，但是在rust里面，翻译成不安全可能不太妥。因为他里面的代码，只是部分逻辑不经过编译器检查(注意是部分)，代码安全不安全是靠程序员来保证，并不是这里面的代码一定不安全。

用法
====

unsafe可以用于以下场景,且即以下场景在unsafe块里不会被检查，但类似引用或者生命周期等还是会被检查:

-   解引用裸指针
-   调用unsafe的函数或方法
-   访问/修改可变静态变量
-   实现unsafe trait
-   访问union字段

解引用裸指针
------------

`*const T` 和 `*mut T` 被称为裸指针.如

``` {.rust}
// 创建指向任意地址的裸指针
let address = 0x012345usize;
let r = address as *const i32; 

// 不能保证解引用是合法的,所以解引用的时候可能会出错
unsafe{
    println!("{:?}",*mut_r)
}


// 其他创建方式
let num = 6;
let r = &num as *const i32; // 不可变裸指针
let mut num = 8;
let mut_r = &mut num as *mut i32; // 可变裸指针

// 因为创建的时候就是合法的，所以解引用的值也是合法的
unsafe{
    println!("{:?}",*mut_r)
}
```

创建一个裸指针没有任何风险，只有访问其指向的值时，才可能会遇到无效的值。

调用unsafe函数/方法
-------------------

``` {.rust}
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

访问/修改可变静态变量
---------------------

不可变静态变量是安全的，但是可变静态变量是不安全的，可能会有多个线程同时访问,当有多个线程对该数据进行竞争时，需要程序员自己去处理竞争。

``` {.rust}
static mut NUM :i32 = 1;

fn main(){
    // 访问
    unsafe{
        println!("{:?}",NUM);
    }

    // 修改
    unsafe{
        NUM += 1;
    }
}
```

unsafe trait
------------

当至少有一个方法中包含编译器不能验证的不变量时 trait 是不安全的.

``` {.rust}
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
```
