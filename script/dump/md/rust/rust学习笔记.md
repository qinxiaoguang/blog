书籍
====

<https://rustcc.gitbooks.io/rustprimer/content/>

install
=======

1.  `curl https://sh.rustup.rs -sSf | sh`
2.  `rustup toolchain add nightly`
3.  `cargo +nightly install racer`
4.  `git clone https://github.com/rust-lang/rust.git ~/.rust`
5.  `export RUST_SRC_PATH="/path/to/rust/src"`
6.  emacs环境: `(setenv "RUST_SRC_PATH" "/path/to/rust/src")`

update
======

`rustup update` : stable和nightly版本更新 `rustup install nightly`
nightly版本更新 `rustup self update` rustup升级
`rustup run nightly rustc --verion` 查询版本 `rustup default nightly`
选择nightly版本

emacs配置
=========

``` {.elisp}
(use-package rust-mode :ensure t :defer t
  :config
  (use-package racer :ensure t
    :config
    (setq racer-cmd "/Users/qinxiaoguang01/.cargo/bin/racer")
    (setq racer-rust-src-path "/Users/qinxiaoguang01/.rust/src"))
  (use-package company-racer :ensure t
    :config
    (setq company-racer-executable "/Users/qinxiaoguang01/.cargo/bin/racer"))
  (use-package flycheck-rust :ensure t)
  (add-to-list 'auto-mode-alist '("\\.rs\\'" . rust-mode))
  (add-hook 'rust-mode-hook
            '(lambda ()
               ;; Enable racer
               (racer-activate)
               ;; Hook in racer with eldoc to provide documentation
               (racer-turn-on-eldoc)
               ;; Use flycheck-rust in rust-mode
               (add-hook 'flycheck-mode-hook #'flycheck-rust-setup)
               ;; Use company-racer in rust mode
               (set (make-local-variable 'company-backends) '(company-racer))
               ;; Key binding to jump to method definition
               (local-set-key (kbd "M-.") #'racer-find-definition)
               ;; Key binding to auto complete and indent
               (local-set-key (kbd "TAB") #'racer-complete-or-indent))))

```

Hello world
===========

``` {.rust}
fn main(){
    println!("hello world");
}
```

编译: `rustc file.rs` 运行: `./file`

Hello Rust
==========

-   创建项目 `cargo new hellorust --bin`
-   项目目录
>.<br>
├── Cargo.toml<br>
└── src<br>
    └── main.rs<br>

-   编译 `cargo build` ,优化编译 `cargo build --release`
-   运行 `./target/debug/hellorust`
-   编译并运行: `cargo run` or `cargo run --release`

变量
====

-   rust中变量赋值强调的是变量绑定，而非赋值
-   rust中的变量必须有初始值,其无默认值
-   rust
    中允许声明同样名字的变量，后面的会将前面的遮挡，不过此时内存中有两个变量，只不过只能使用一个。

``` {.rust}
//varable
let a1 = 5; // int32类型，rust会做类型推断, 没有mut关键字，都是不可变变量
//a1 = 6 ; 不可变变量，若改变其值会报错
let a2:i32 = 5; // int32类型,注: rust中的变量名都很短
let b1:u32 = 5; // unsign int32类型

let mut a11:f64 = 1.0; // 可变变量
a11 = 2.0; // 不会报错
let a11 = a11; // 可重新绑定为不可变
let b12 = 1.2f32; // rust中的value + type写法

let (a, mut b):(bool, bool) = (true, false); // 多个变量赋值
let (mut a, mut b) = (1,2) // 声明多个可变变量

let v = vec![1,2,3];
let mut v = v; // 可以通过创建同名变量的方式改变其可变性,别犹豫,绝对正确.

// 静态变量/全局变量
// 必须初始化,且必须是可确定的常量
static GLOBAL : i32 = 0;
// 带有mut的全局变量，改变的时候必须有unsafe关键字修饰
static mut G2 : i32 = 1;
unsafe {
    G2 = 5;
}
// 常量, 因为其为常量，所以不允许有mut关键字修饰
const GLOBAL : i32 = 0;
```

类型
====

-   布尔类型：有两个值true和false。
-   字符类型：表示单个Unicode字符，存储为4个字节。可将来任何字符赋值, 如
    `let c='秦'`,也可以使用u8来类型来存储ASCII字符，如 `let x :u8=b'A'`
-   数值类型：分为有符号整数 (i8, i16, i32, i64, isize)、 无符号整数
    (u8, u16, u32, u64, usize) 以及浮点数 (f32,
    f64)。其中iszie,usize则是自适应类型，其大小取决于操作系统。
-   字符串类型：最底层的是不定长类型str，更常用的是字符串切片&str和堆分配字符串String，
    其中字符串切片是静态分配的，有固定的大小，并且不可变，而堆分配字符串(String)是可变的。
```{.rust}
let hello = "hello world"; // 双引号中的字符串类型为&'static str, 即其不可变
let hello : &'static str = "hello world"; // 两种方式等价
// String 类型，类比[T]和Vec<T>的关系，str和String就是这种关系
let mut s = String::new();
let mut hello = String::from("hello");
hello.push('w'); // 压入字符
hello.push_str("orld"); // 压入字符串
hello.pop(); // 弹出
// str转String
let x:&'static str="hello";
let mut y:String = x.to_string();
// String 转str
let s = "Hello".to_string();
let ss = &*s;
// 可使用r来避免字符串转义
let d &'static str = r"abc/nabc";
// 下标访问
let c="hello".to_string();
c.chars().nth(2); // 访问第2个 
// 字符串切片，很危险，不建议使用
let s = String::from("haha");
let s1 = &s[1..2]; // s1是&str类型，&str就是切片类型, 使用&s[..]可以把String类型转为&str类型
let s2 = "haha";   // s2也是&str类型, 所以s2和&s2[..]是一样的
let s3 = String::from("哈");
println!("{}", s3.len()); // 打印3， 因为其字符串使用的utf-8存储，所以一个哈字使用3个字节存储，所以打印3，注意String底层实际上是Vec<u8>，所以其长度也即为Vec<u8>的长度/字节长度，因此为了防止误会，杜绝了对字符串使用索引的操作，如s3[0]是不会编译通过的。而对字符串使用slice操作，如&s3[0..4]通常是很危险的，因为不知道会截取出来什么乱七八糟的玩意,也有可能导致panic的出现
// 操作
let s1 = String::from("haha");
let s2 = String::from("heihei");
let s3 = s1+&s2;  // 使用+号对String操作时，第一个要是String类型，第二个是&str类型,或者&String也可以,因为&String类型会被强转成为&str类型(deref),但是这样操作后s1将会被move, 生成的s3是String类型
let s3 = s1 + &s2 + &s2; // 合法
let s3 = s3 + "haha"; // 可以
let mut s4 = String::from("haha"); 
s4.push_str(" oo"); // 可以使用push_str来给字符串后边添加新的字符串
s4.push('l'); // 可以使用push 来添加字符
// 遍历
// s.chars()也是iterator类型，所以可以有iterator的一系列操作
let s = String::from("哈黑");
for i in s.chars(){
     // 使用chars才能获取正确的字符, 而使用s.bytes()为所有字节
}
let count = s.chars().count(); // 获取字符串的长度，注意是不是字节长度
let two = s.chars().nth(2); // 获取第2个字符
let back_two = s.chars().back_nth(2); // 获取倒数第二个字符
// 字符串替换
let res = str::replace("haha!","!","?");
let res = res.replace("?","!");
```

-   数组：具有固定大小，并且元素都是同种类型，可表示为\[T; N\]。

    ``` {.rust}
    let array : [i32; 3] = [0; 3]; // 数组大小是固定的，[T; N]用来表示数组N个T类型,[0;3]表示3个0构成的数组
    for x in &array {
        println!("{}", x)
    }

    // vector
    let v:Vec<i32> = Vec::new(); // 空vector, 不可变，不可压入数据
    let v:Vec<i32> = Vec![]; // 宏创建
    let v = Vec![1,2,3,4,5];
    let v = vec![0;10];
    let mut v = vec![1,2]; // 可变vector,可压入数据
    v.push(3); // 压入数据
    let res = v.pop(); // 弹出数据
    ```

-   切片：引用一个数组的部分数据并且不需要拷贝，可表示为&\[T\]。&符号不要考虑成引用，会给自己增加负担，相反，应考虑为切片的必要的一部分。
```{.rust}
let arr = [1,2,3,4,5];
let slice_1 = &arr[..]; // 获取全部元素, 注意是引用,原来的元素如果修改，则引用的元素也会修改
let slice_2 = &arr[1..4]; // 获取下标[1,4)的元素
let slice_3 = &arr[1..]; // 获取下标1之后的所有元素
let slice_4 = &arr[..3]; // 获取下标3之前的所有元素
let slice_5 = &arr[1..=4]; // 获取[1,4]的元素
// 有关slice的函数
fn show(arr: &[u8]){
    for i in arr {
        print!("{} ",i);
      }
}
// 调用
show(&arr[..]);
show(&arr); // 可以
show(slice_1);
```

-   元组：具有固定大小的有序列表，每个元素都有自己的类型，通过解构或者索引来获得每个元素的值。
```rust
let y = (0, "1234");
let x :(i32, &str) = (3, "123456")
// 若元组只包含一个元素，需要在元素末尾添加逗号，以区分括号表达式
let z = (0,);
// 访问
let (w, z) = y;
// 下标访问
let f = x.0;
let e = x.1;
```

-   指针：最底层的是裸指针\*const T和\*mut
    T，但解引用它们是不安全的，必须放到unsafe块里。
-   函数：具有函数类型的变量实质上是一个函数指针。
-   元类型：即()，其唯一的值也是(), 也称单元类型。
-   结构体
    ```rust
    // 1. 通常驼峰命名
    // 2. 结构体的中的值默认不可变,且不支持域类型为可变,可通过Cell来模拟
    // 3. 结构体域的结尾是逗号,
    // 4. 结构体的域默认私有, 可通过pub关键字公开
    struct Point{
        x:i32,
        y:i32,
    }
    
    // 1. 元组结构体,用()来包裹域,且域无名字
    // 2. 通常驼峰命名
    // 3. 元组结构体的构造方法可被当做函数传入
    struct Color(u8,u8,u8);
    // 4. 若元组结构体只有一个域，则其为newtype
    struct NewInt(i32);
    // 5. 空结构体占用空间为0
    struct D;
    
    fn main() {
        let point = Point{x:1, y:2};
        let point2 = Point{..point}; // .. 表达式可以表示copy
        let point3 = Point{x:2,..point}; // .. 表达式可以表示copy
        println!("{}",point.y)
    }
    ```

类型别名
--------

-   可以使用type为一个类型起一个别名,且这两个类型一模一样，只不过名字不一样，不要和golang混淆
-   泛型其实就是使用类型别名的方式实现的。
``` {.rust}
type Age = u32;
```

类型转换
--------

-   类型转换的方式是通过as关键字
-   如果转换是合理的，则编译通过，否则编译不通过
``` {.rust}
let var1 : i8 = 41;
let var2 : i16 = var1 as i16;
let i = "haha";
let b = i as u32; // 不合理，编译错误
```

输出格式化
==========

``` {.rust}
println!("{}", 1);
println!("{:o}", 9); // 8进制
println!("{:x}", 255); // 16进制
println!("{:X}", 255); //16进制大写
println!("{:p}",&0); // 指针
println!("{:b}",15); // 2进制
println!("{:e}",100000f32); //科学计数
println!("{:?}","test"); //打印Debug trait
println!("{:#?}", ("test1","test2")); // 带换行和缩进的Debug打印
println!("{a} {b} {b}", a="x", b="y"); // 带命名参数的打印
```

控制流
======

if
--

``` {.rust}
// 形式1 
if expr1 {

}

// 形式2
if expr1 {

} else if expr2 {

} else {

}

// 形式3
if expr1 {

} else {

}

// 如果使用if-else作为表达式，则其分支中返回的类型必须一致，若else分支省略了，则编译器默认认为else分支的类型为(), 所以下面的写法是错误的
fn test(flag:bool)->i32{
    if flag {
        42
    }
}

// rust 中if是一个表达式,so可以这么写
let x = 5;
let y = if x == 5 {10} else {15};
// rust基于表达式的语言，有且仅有两种语句，1. 声明语句:如let, 2. 表达式语句,在表达式后加';',将表达式变成语句。
// 以;结尾的为语句,语句的返回值为unit (),如 x=5 是一条表达式，而x=5; 是一条语句

// if let是match的简化用法
let x = Some(5);
if let Some(y) = x{
    println!("{}", y);
}
let z = if let Some(y) = x {y} else {0}; // 若x中有值，则赋给z
```

for
---

``` {.rust}
// expression 是一个迭代器,如0..10,or [0,1,2].iter()
for var in iterator {

}
// eg :
for x in 0..10 {

}

// 获取索引，使用enumerate()函数
for (i,j) in (0..10).enumerate() {
    // 注意j是引用
    println!("i is {}, j is {}", i, j);
}
```

while
-----

``` {.rust}
// expr是一个bool的表达式
while expr{

}

// loop类似 while true,一般推荐用loop,其有优化.
loop {

}

// loop中的break后跟一个表达式，则该表达式的值就为loop表达式的值,如
let v = loop { break 10;}; //此时v的值为10
```

match
-----

``` {.rust}
// match 类比switch,需符合以下要求
// 1. 语句以,结尾
// 2. 若要获取匹配值，使用@符号,通常用于模式匹配中
// 3. 必须要有_符合来穷举剩余匹配，因rust要求覆盖所有可能
let mut cnt = 0;
match cnt {
    0|6 => println!("ha"),
    e @ 1 ... 5 => println!("hei"),
    _ => println!("huhu"),
}
// 4. 可通过ref关键字得到某值的引用
match cnt {
    ref r => println!("got a ref {}", r),
}
// 5. 可模糊匹配元组
let pair = (0, -2);
match pair {
    (0, y) => println!("y is {:?}", y),
    (x, 0) => println!("x is {:?}", x),
    _ => println!("doesnt match anything"),
}
// 6. 可匹配结构体,及枚举
let origin = Point {x:0, y:0};
match origin {
    Point{x, ..} => println!("x is {}", x),
}
// 也可对struct中的域进行重命名
match origin {
    Point{x:x1, y:y1} => println!("x is", x1),
}
// 7. 除了panic,所有分支的表达式的结果必须相同
// 后置条件
let y = false;
match x {
    4|5 if y => println!("yes"), // 其实就是 (x in 4|5) && (if y)
    _ => println!("no"),
}
// 其实上述内容和haskell中的模式匹配基本一样
// 也可通过let和while进行模式匹配
let number = Some(7);
if let Some(i) = number {
    println!("i is {}", i);
} else {
    println!("doesnt match a number");
}
// while let
while let Some(i) = number {
    if i > 5 {
        println!("i is gt 5");
        break;
    } else {
        println!("i is small than t");
    }
}
```

函数
====

``` {.rust}
// fn 开头，可以多个参数，最多一个返回值
fn add_one(x: i32) -> i32 {
    // 若以;结尾，则返回()
    x + 1
}

// 但是可以利用元组来达到返回多个值的效果
fn get_two() -> (i32,i32){
    (1, 2)
}

// 发散函数，使用!作为返回类型,其实和golang的panic或者java的exception一个意思
fn diverging() -> !{
    panic!("this function will never return");
    }
//发散函数返回值可以赋值给任何类型
fn test() -> i32{
    diverging()
}

// 函数也可以使用模式匹配
fn print_id((_, age):(&str, i32)) {
    println!("my age is {}",age);
}

// 若函数不加返回值，则其返回(), 即fn main() 和 fn main() -> () 是等价的。
fn main(){
    // 匿名函数
    // rust中的函数，最后一个表达式的值默认作为返回值。
    let plus_num = |x:i32| x+num;
}

// 使用impl关键字来为enum,struct等类型创建方法
struct Person {
    name :String,
}

impl Person {
    fn new(n: &str) -> Person{
        Person {
            name: n.to_string(),
        }
    }

    fn greeting (&self){
        println!("{} say hello.", self.name);
    }
}

// 函数作为一等公民，可通过type为某个函数类型定义别名
fn inc (n : i32) -> i32{
    n + 1
}
type IncType = fn(i32) -> i32;
let func: IncType = inc;
func(3);

// 函数作为返回值
fn get_func(n: i32) -> fn(i32) -> i32{
    // rust 支持在函数中定义函数，但是不能使用该函数外的变量，若要使用，则需要使用闭包
    fn inc(n:i32) -> i32{
        n+1
    }
    inc
}

// 函数可以用const关键字修饰，这样函数可以在编译阶段执行，返回值也被视为编译器常量,如
#![feature(consts_fn)]
const fn cube(num :usize) -> usize {
    num*num*num
}
```

测试
====

``` {.rust}
// 单元测试，需要将tests模块放入相同的文件里，并用#[cfg(test)]标注, #[cfg(test)]注解告诉Rust,只在运行cargo test的时候才执行, 而在cargo build的时候不执行
#[cfg(test)]
mod tests {
     #[test] // 需要使用#[test]注解来表明该函数是测试函数, 使用cargo test来运行测试函数
     #[should_panic] // 测试panic,若有panic成功，若无panic失败
     fn it_works() {
          assert_eq!(2 + 2, 4);  // 测试相等
          assert_ne!(2 + 1, 4);  // 测试不等
          assert!(true); // assert!参数传递为bool
          assert!(false,
               "自定义输出信息 {}", "哈"
          )
     }

     #[test]  // 通过Result来判断测试结果
     fn get_result() -> Result<(), String>{
          if true{
               Ok(())
          } else {
               Err("hei".to_owned())
          }
     }
}

// 集成测试需要在和src的同级目录下创建一个tests目录,例如创建一个main_test.rs文件，里面不需要#[cfg(test)]及mod tests做标注， 同样该文件也只有在运行cargo test的时候才会执行
```

trait
=====

类似golang的interface,但有很大不同

``` {.rust}
trait HasArea {
    fn area(&self) -> f64; // 第一个参数使用&self,则其称为方法，可以使用小数点调用，若没有&self，则为静态函数，可以使用::来调用
    // self, Self都是关键字，其中Self是类型，self是变量名, 
    // self 也分几种类型，如self, &self, &mut self，他们都是self:Self， self:&Self, self &mut Self 的简化版本

    // 默认方法，无须实现,也可重写
    fn default(&self) -> f64{3.14f64}
}
// 实现
struct Cycle{
    x : f64,
    y : f64,
    radius: f64,
}

impl HasArea for Cycle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

use std::fmt::Debug;
// 泛型约束,T必须实现Debug
fn foo<T: Debug>(s: T) {
    println!("{:?}", s);
}

use std::fmt::Debug;
// 泛型约束, T必须实现Debug和Clone两个trait
fn foo<T: Debug + Clone>(s: T) {
    s.clone();
    println!("{:?}", s);
}

//利用 where 从句简化/美化代码
fn foo<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

// 或者
fn foo<T, K>(x: T, y: K)
    where T: Clone,
          K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

// trait 继承
trait Foo {
    fn foo(&self);
}

trait FooBar : Foo {
    fn foobar(&self);
}

// 但实现时比较特殊
struct Baz;

// 只能实现自己里面有的方法
impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
}

impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
}

// 可以为内置类型添加一些方法，如为i32添加方法
impl SomeTrait for i32 {
    fn xxx
}

// trait不可被当做参数传递,如下列代码是错误的
fn test(args: HasAres) -> i32{}
// 若想实现以上效果，需要使用泛型来实现
// derive 和haskell中一样
// 可以不必手动的实现一些trait，通过derive来派生
// 能被derive的trait有:Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd
#[derive(Debug)]
struct Foo;
```

泛型
====

-   泛型可以有默认值

    ``` {.rust}
    struct S<T=i32>{
        data:T
    }  
    ```

-   如果指定泛型参数，就必须被使用，否则编译不通过
-   泛型用于函数中

    ``` {.rust}
    fn test<T>(args:T) -> bool{}
    fn test<T1,T2> (args1 :T1, arg2:T2) -> bool {} // 多个泛型参数
    ```

-   可以通过泛型来实现函数重载功能，但是rust中没有同一个函数不同数量参数的函数重载
-   impl中的泛型,在impl块中出现的泛型参数，需要在impl关键字后边用尖括号声明
    ```rust
    impl<T,U> Into<U> for T
        where U: From<T>
    {
        fn into(self) -> U {
            U::from(self)
        }
    }

-   泛型参数约束有两种方式，1. 在泛型参数声明时使用冒号 2.
    使用where子句,
    使用冒号的方式一定可以转成where，但使用where的却不一定能转成冒号

    ``` {.rust}
    // 1.
    fn max<T: Ord>(a: T, b:T)->T{}
    // 2.
    fn max<T>(a:T, b:T)->T
       where T: Ord
    ```

-   关联类型，也是泛型参数如Iterator中有一个关联类型为Item，在使用的时候可以通过名字进行关联
    ```rust
    trait Test{
        type N; // N即为一个关联类型
        fn test(&self) -> Self::N;
    }
    
    // 如果要实现的话
    impl Test for i32{
        type N = i32;
        fn test(&self) -> i32 {}
    }
    ```

-   一般来说，在尖括号里存在的是输入类型参数，在trait内部存在的关联类型是输出类型参数

所有权
======

1.  rust中没有变量的概念，而是标识符和资源的概念
2.  rust中变量不会初始化默认值
3.  `a=100` ,表示让100这个资源和a这个标识符进行绑定
4.  离开作用域后，作用域中的变量绑定的内存(无论是否是常量),以及所有者变量一起被销毁.
5.  使用let可以把资源所以权从一个绑定转移到另一个绑定,
    `let 标识符A = 标识符B`
    表示把B绑定的资源的所有权转移给A,转移后,A不绑定任何内容,
    若继续使用就会报错.如:

    ``` {.rust}
    let a = 1i32;
    let b = a; // 1i32此时被绑定给b， a无绑定值
    println!("{}",a); // 报错

    // 第二个例子
    let v = vec![1,2,3];
    fn take(v:Vec<i32){}
    take(v);
    println!("{}",v); // 报错, v指向的资源所有权已经被重定向给函数take中的变量。可以理解为执行take(v)时候，先进行了资源绑定
    ```
> Before move:<br>
a \<=\> 内存(地址：A，内容：\"xyz\")<br>
After move:<br>
a<br>
b \<=\> 内存(地址：A，内容：\"xyz\")<br>

6.  rust中规定，一个资源同一时刻只有一个owner.
7.  若被move的变量实现了Copy，那么move时候会拷贝资源到新的内存取余，并把新的内存区域内容binding给新变量,在rust中,基本数据类型均实现了Copy特性.
>Before move:<br>
a \<=\> 内存(地址：A，内容：100)<br>
After move:<br>
a \<=\> 内存(地址：A，内容：100)<br>
b \<=\> 内存(地址：B，内容：100)<br>

8.  基本类型的浅拷贝和深拷贝的作用一样,
    浅拷贝可以理解为仅仅拷贝了内存地址。而String类型若要实现深拷贝，则需要使用Clone特性。

    ``` {.rust}
    let a: String = String::from("xyz");
    let b = a.clone();  // <-注意此处的clone
    ```

9.  rust中不使用mut修饰的变量为不可变变量，这个不可变变量的意思是绑定不可变。绑定不可变的变量不允许再次绑定且不允许修改资源内容。使用mut可以将其变更为可变变量，可变变量允许重新绑定，且允许修改绑定的内容。
10. 若一个struct中的所有域的类型都实现了Copy特性，那么此类型就可以实现Copy特性,否则不能通过derive派生实现。
11. move关键字常用在闭包中，强制闭包获取所有权
12. &符号表示borrowing,其不会发生所有权move,如
    `let x:Vec<i32> = vec!(1i32,2,3); let y = &x`, borrowing的规则为
    -   同一作用域，特定数据最多只有一个可变借用（&mut T），或者2。
    -   同一作用域，特定数据可有0个或多个不可变借用（&T），但不能有任何可变借用。
    -   借用在离开作用域后释放。
    -   在可变借用释放前不可访问源变量。
13. borrowing也分不可变借用(&T)和可变借用(&mut
    T)，不可变借用只读,而可变借用可读写，但其借用的对象也要有可变性.但要注意区分mut修饰变量和修饰借用时的区别

    ``` {.rust}
    let p = &mut v; // 表示p是对v的可变借用,但是p无法重新绑定
    let mut p = v; //表示p是可以重新绑定的
    // mut修饰变量时强调的是绑定关系。
    ```

14. 可变借用未释放时，原变量无法访问

    ``` {.rust}
    let mut x = 1_i32;
    let p = &mut x;
    x = 2;  // 编译报错，因x已被借用，无法访问和修改
    println!("{}", x);
    ```

15. `let y = &mut x` 和 `let mut y = &mut x`
    的区别是，第二个y还可以被可变借用，而第一个则不可被可变借用。
16. `'a` 是lifetime的标识符,类比泛型中的T名字。

生命周期
========

1.  生命周期符号使用单引号开头，与泛型类型的参数是一样的，都是先声明后使用。

    ``` {.rust}
    fn test<'a>(arg: &'a T) -> &'a i32{

    }
    ```

2.  若生命周期a比生命周期b长，则记为 `'a:'b`
3.  \'static是一个特殊的生命周期，表示程序从开始到结束的整个阶段
4.  在实际调用的时候，生命周期会被特化为对应的参数，如

    ``` {.rust}
    fn test<'a>(arg1:&'a i32, arg2 : &'a i32) -> &'a i32{}
    let x = 1;
    let y = 2;
    test(&x, &y); // 虽然x和y的生命周期不相等，但是其只要满足 x:'a, y:'a即可行
    ```

5.  若自定义类型中的成员包含生命周期参数，那么该类型也必须有生命周期参数

    ``` {.rust}
    struct Test<'a> {
          member :&'a str
    }
    // 实现的时候也要先声明生命周期,其中impl后跟的泛型是声明，而Test后跟的泛型是使用
    impl <'a> Test<'a> {
         fn Test<'a>()
    }
    ```

6.  生命周期是可以省略的，其对省略的生命周期会自动补全，补全规则为
    1.  每个带生命周期参数的输入参数，对应不同的生命周期参数
    2.  如果只有一个参数带生命周期参数，那么返回值的生命周期也被指定为该参数
    3.  如果有多个参数，但其中有&self, &mut
        self,那么返回值的生命周期被指定为该参数
    4.  若以上都不满足，则不能补全

    ``` {.rust}
    fn get_str(s: &String) -> &str // 可自动补全，符合条件2, 所以其补全为
    fn get_str<'a>(s:&'a String) -> &'a str
    ```

析构
====

-   两种方式执行析构.1. 手动调用std::mem::drop()方法 2.
    实现std::mem::Drop trait.
-   其中std::mem::drop()方法是一个空的函数体,其执行时实际是调用move语义。且参数只能是T类型，而不是&T类型。
-   实现std::mem::Drop的trait后，生命周期结束后会自动执行对应的方法。不允许手动调用。且该trait的drop方法的参数类型是&mut
    Self
-   使用下划线绑定一个变量，这个下划线对应的变量会当场执行析构
-   实现Drop的同时不能实现Copy

智能指针
========

智能指针多了两个trait,
Deref用来解引用，使得智能指针可以直接对指向的内容进行操作，Drop用来析构

内部可变性
----------

rust在不使用mut来修饰变量时候，其内部是不能发生改变的，比如
`let v = vec![1,2,4]`
此时对v使用v.push方法是会报错的，而内部可变性是说，在不使用mut来修饰变量时，也可改变内部状态，如Cell

Box
---

Box将数据存储在堆上，而非栈上, 一般用于以下情形

1.  递归数据结构，如链表，二叉树,Box有已知的大小
2.  与dyn配合使用,dyn粗粒度理解为多态，即无法明确其具体类型时，如Box\<dyn
    Animal\>

``` {.rust}
// 1.递归数据
pub struct List{
     // 首先明确为什么用Box，因为不用Box,则指定为List类型，List类型是递归类型，所以无法明确其大小，而使用指针可以明确大小
     // 其次明确为什么使用Option,因为next可能为空，而rust没有空的概念，要表示空的概念，需要使用Option类型.
     next:Box<Option<List>>,
     value:i32,  // 以i32举例
}
// 创建的方式
let l = List{
     next: Box::new(None),
     value: 18,
};
```

Cell
----

-   Cell类似一个壳,把数据包裹在里面，所有指针只能指向Cell，不能指向数据,修改数据只能通过Cell来完成,用户无法创建一个直接指向数据的指针
-   以下代码是可以编译通过的

``` {.rust}
use std::cell::Cell;

fn main() {
    let data = Cell::new(1); // 没有使用mut修饰
    data.set(2); // 可改变内部状态
    let _p = &data; 
    println!("{:?}",data); // 在存在p引用的时候也可访问
}
```

RefCell
-------

-   RefCell是另外一个提供内部可变性的类型,Cell无法提供一个直接指向内部数据的指针，而RefCell可以,RefCell用于单线程场景
-   如代码

``` {.rust}
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1,2,3]);
    let p1 = &data;
    let p2 = &data;
    p1.borrow_mut().push(4); // borrow_mut可以获取一个指向内部数据的可变指针，向vec这样的数据，只有获取到了指针才能做修改
    p2.borrow_mut().push(5);
    println!("{:?}",data);
}
```

解引用
------

-   实现std::ops::Deref或std::ops::DerefMut
-   解引用是编译器的一种自动机制,在某些场景会隐式的启动
-   通过解引用可以方便的直接对其内部的数据进行操作，而不用先获取内部数据的指针再进行操作
-   deref方法返回的是&T而不是T。
-   针对\*y，会自动转换为\*(y.deref())
-   如String类型实现了向目标类型str的解引用的转换

    ``` {.rust}
    fn main() {
        let s = "hello";
        println!("{:?}",s.len());
        println!("{:?}",(&s).len());
        println!("{:?}",(&&s).len());  //均能打印出结果
    }

    // len的函数是
    fn len(&self) -> usize
    // 那么为什么使用 (&&&&s).len()也能调用呢，原因是在(&&&&s)中找不到len()方法，则尝试他的deref,变成(&&&s)，再寻找len方法，依次类推，直到找到为止。
    ```

-   如Rc实现了Deref

    ``` {.rust}
    use std::rc::Rc;

    fn main() {
        let s = Rc::new(String::from("haha"));
        println!("{:?}",s.bytes());  // 此处进行了解引用,获取到了String类型，而String类型没有bytes方法，所以其也进行解引用，获取到了&str类型，接着就可以调用bytes方法
    }
    ```

-   Vec\<T\>也实现了Deref,目标类型是\[T\]，所以可以通过deref来获取&\[T\]的切片
-   若s实现了Deref，则 `&*s` 等效与s.deref()
-   若某个结构存在方法A(),而其deref也存在方法A()，那么要调用deref的方法A()，只能手动调用
    ```rust
    // 以下代码是编译不通过的,原因是&s是String类型，而分支中的"hehe"是&'static str类型,此时编译器并没有对s进行自动deref
    let s = String::from("hehe");
    match &s {
        "hehe" => println!("{:?}","hehe"),
        _ => println!("{:?}","heihei"),
    }
    // 此时需要我们自己deref,deref的方式有如下
    // 1. s.deref()
    // 2. &*s
    // 3. s.as_ref()
    // 4. s.borrow()
    // 5. &s[..]
    // 所以实现字符串匹配可以这么写
    match s.deref(){
        "hehe" => println!("{:?}","hehe"),
        _ => println!("{:?}","heihei"),
    }
    ```

-   解引用强制多态，
    是说当一个变量传递给一个函数时，若该函数接收的类型不是传入的类型，会强制的通过deref来解引用为需要的类型，如函数`fn test( s:&str)`需要的是`&str`类型，但是传入`&String`也可以，因为`&String`会被强制解引用为&str类型,
    也可以传入`&Box::new("haha".to_string())`类型

引用计数
--------

-   std::rc::Rc和std::rc::Arc类型都提供了引用计数功能，一块数据当所有引用都消亡时，该数据的内存才会释放
-   Rc类型的引用计数是普通整数操作，只能在单线程中，而Arc的引用计数是原子操作，可以在多线程中
-   其创建引用的方式使用clone，如

    ``` {.rust}
    let s = Rc::new(String::from("hehe"));
    let r1 = s.clone();
    let r2 = s.clone();

    let r3 = Rc::clone(&s); // 但是一般建议使用这种方式，与*.clone()方法区别出来
    println!("{:p} {:p}",&*r1, &*r2);  // &*s 相当于s.deref() ,打印出来的地址是一样的,说明r1和r2指向的数据是同一个
    ```

unsafe
======

-   unsafe可用于修饰函数fn, 代码块，trait,impl等
-   unsafe具有传递性，具有unsafe性质的函数，其调用者也必须有unsafe修饰
-   Rust中提供了两种裸指针，\*mut T和\*const T，可以通过\*mut
    T修改所指向的数据，而\*const T不能，但在unsafe中，这两个指针可以互换
    -   裸指针可以为空，且不保证裸指针一定指向一个合法的地址
    -   裸指针不会执行任何自动化清理工作
    -   裸指针复制操作执行的是简单的内存浅复制
    -   创建裸指针是安全的，只有在对裸指针解引用才是不安全的，如

        ``` {.rust}
        let mut y : u32 = 1;
        let raw_mut = &mut y as *mut u32 as *mut i32 as *mut i64; //安全的, 可以通过as来进行类型转换，转换后就可以当做另一种类型来进行操作, 但是需要注意的是，如果将u32强转为i64,在对其进行赋值的时候，就会影响到本来不属于u32的内存地址
        unsafe {*raw_mut = 2;} // 不安全的，必须在unsafe块中
        println!("{:?}",y);
        ```

-   标准库中有个std::intrinsics模块，该模块的函数是在编译器内部实现，在使用的时候需要使用unsafe来修饰，这些函数不是准备直接提供给用户使用的
    -   transmute函数可以执行强制类型转换，把一个T类型参数转换为U类型返回值,但其内部的二进制值不变,且必须满足两者的size是一样的。其实我们也可以通过as来进行实现，但是不能实现size一样的约束条件
    -   transmute_copy函数是对引用进行的copy操作，其参数是引用类型，而transmute参数是T类型,且有move语义,如查看Vec的内存表示

        ``` {.rust}
        let v = vec![1,2,3];
        unsafe {
            let t : (usize,usize, usize) = std::mem::transmute_copy(&v);
            println!("{} {} {}", t.0, t.1, t.2);
        }
        ```

闭包
====

-   匿名函数，具有捕获外部变量的能力,也被称为lambda表达式
-   其特点是访问外部变量，而函数不可访问外部变量
-   匿名函数可以省略类型，编译器会根据上下文环境自动推倒，但是同一个匿名函数不能有两种语义。
    ```rust
    let add = |x:i32, y:i32|->i32 {return x + y};
    let add = |x,y|x+y;
    ```

-   使用move关键字来修饰闭包，可以将闭包中使用的外部变量自动move，并可以将对应的闭包传递到函数外部,通常用于将变量传递到函数外部
-   Fn/FnMut/FnOnce, FnOnce对应的self是self,FnMut是&mut self,
    Fn是&self，对于一个闭包，他会尽量impl Fn，依次尝试impl FnMut,
    FnOnce,这些都是编译器自动分析出来的。
-   举个例子
    ```rust
    let v = vec![1,3,3];
    let d = ||drop(v);// 其中std::mem::drop(d:T)中参数是T类型，所以v是会被move到闭包中,那么他对应的trait是FnOnce,因为Fn和FnMut都行不通(都需要&self引用，但是闭包中并没有v的引用),所以生成的闭包只能调用一次。
    d();
    d(); // 调用失败
    
    // 同理，生成Fn的方式也和闭包使用的外部变量的方式有关，如
    let v = vec![1,3,3];
    let d = || for i in &v{println!("{:?}",i)};
    d();
    d(); // d闭包使用的外部环境v是引用类型，所以其对应的trait是Fn，对应的self是&self,不存在move语义，所以其可以调用多次
    ```

-   每个闭包，编译器都会为其生成一个匿名结构体类型
-   静态分派/动态分派
    ```rust
    // trait可以返回，但是不能直接返回，如有trait Animal,那么函数不能这么写 fn test()->Animal
    // 因为编译器不知道Animal占用多少空间,即不知道trait占用多少空间，但是有以下两种写法
    // 1. 静态分派，表示返回的trait具体是哪个，在编译时期就确定了下来， 其使用泛型及impl trait来完成
    fn test(arg:Animal) // 入参可以直接使用Animal来表示，这种是静态分派·
    fn test() -> impl Animal // 表示返回的类型实现了Animal的trait
    // 2. 动态分派，表示具体调用的trait是在执行阶段才能确认
    fn test(arg:Box<dyn Animal>) // 虽然trait不知道空间，但是Box可以知道,Box中传入一个trait object,这种就属于动态分派，在运行期确定调用的哪个函数,dyn是一个关键字，目前未稳定，表示trait中的具体类型是动态的
     
    ```

-   trait object,例如trait Animal ,那么dyn Animal
    就是一个动态大小类型(DST),而&dyn Animal, &mut dyn Animal, Box\<dyn
    Animal\>,Rc\<dyn Animal\> 都是trait object

协程
====

生成器
------

-   生成器是协程的基础,生成器和闭包的语法很像，当闭包中有yield关键字的时候，它就不是一个闭包，而是一个生成器。
-   但有yield关键字时，编译器自动为该闭包生成一个方法resume(),要运行该闭包(生成器)时，使用resume()方法
-   yield与return相似，会返回一个值，但是下次调用resume()方法时，会从yield的地方继续运行
-   yield
    返回的值需要使用GenerateState::Yielded(v)来接收，而return的返回值使用GenarateState::Complete()接收,当返回了Complete时候，就不能在调用resume方法

协程
----

-   async/await, async关键字可以修饰函数闭包和代码块:
    ```rust
    async fn f1(arg : u8) -> u8 {} 
    // 等同于,
    fn f1(arg : u8) -> impl Future<Output = u8>
    // await只能在async中出现，其表示的含义是，若异步的程序没有执行完毕，那么其会进行yield，暂时退出该Future,每当调度器恢复其执行，都会通过poll来查看异步程序运行状态，直到运行完毕,
    async fn fetch(client hyper::Client) -> io::Result<String> {
        let res = await!(client.get("http://www.baidu.com"))?;
        if !res.status().is_success(){
            return Err(..);
        }
        Ok("something")
    }
    // 可以看到使用async/await写代码逻辑，与非异步的程序的代码逻辑是类似的 
    ```

-   Future:基于生成器实现，他内部有一个方法是poll,该poll方法用于查看当前协程的运行状态,
    Future具有能在某个状态中断执行的特性，在某个时刻恢复执行的特性，其都是使用yield来实现的.

线程
====

``` {.rust}
use std::thread;
// 创建线程
thread::spawn(move || {
    // 线程内部逻辑
});
let t = thread::spawn(move || {});
t.join() // join()方法可以等待线程执行完毕

// Builder可以为线程指定更多参数信息
thread::Builder::new().name("thread name".to_string()).spawn(move || {});

// trait Send
// 线程中的数据需要满足trait Send
// 如果一个类型可以安全的从一个线程move到另一个线程，那它就是move类型。
// 不包含引用的类型都是Send，因为在move的时候，它和原来的线程就没有什么关系了。
// 典型的Rc就不是Send类型，因为其在被move的时候，不会进行引用+1，而相对的Arc则是Send类型。

// trait Sync
// 表示若T实现了Sync,则在不同的线程使用&T访问同一个变量时是安全的
// 如i32等基础数据类型，都是Sync类型，因为&i32是只读的，多个线程访问没有问题。
// 具有内部可变性且没有考虑线程同步的都是非Sync类型的，如Cell<T>,RefCell<T>
```

管道
----

``` {.rust}
use std::thread;
use std::sync::mpsc::channel;

fn main() {
    // tx 是输入，发送者，rx是接收者
    // 发送者可以有多个，接收者只能有一个,mpsc意为multiple provider single consumer
    let (tx, rx) = channel(); // 使用channel创建的管道是异步管道，其可以无限制的塞数据
    let (tx2,rx2) = sync_channel(0); // 使用sync_channel创建的管道是同步管道，参数指定管道队列的长度，若无法塞数据则阻塞， 和golang的channel的效果一样
    let tx1 = tx.clone();  // 使用clone方法来增加一个发送者
    thread::spawn(move ||{
        for i in 1..10{
            tx.send(i); // 使用tx.send发送值，且该管道没有大小限制，可以一直赛数据，到内存塞满都是可以的
        }
    });


    // 使用rx.recv() 接收管道值，若无值则阻塞
    while let Ok(r) = rx.recv(){
        println!("{:?}",r);
    }
}
```

STD
===

Vec
---

``` {.rust}
fn testVec(){
     let mut v = Vec::new();
     v.push(1);  // 添加数据
     v.push(2);
     v.push(3);
     v.pop(); // 删除数据
     v.first(); // 获取第一个数据，Option<&T>类型
     v.last(); // 获取最后一个数据，Option<&T>类型
     v.remove(0);// 删除下标的元素
     v.get(2);// 获取数据,获取的数据实际上也是引用
     let mut vi = &mut v[1]; // 使用borrow引用获取数据, 注意前一个mut是修饰变量vi的，意为vi是否能重新绑定，或被绑定为mut类型
     *vi = 3;
     println!("{:?}",v);

     let v3= &mut v[..1]; // 这种返回的不是Vec<>类型，而是&mut [i32]类型,所以不能够push数据，只能修改数据
     v3[0] = 5; // 注意 不需要*v3[0] = 5, 因为加下标本身就是一个指针，考虑c++语言中的情形
     println!("{:?}", v);
     // 遍历元素，注意使用borrow,否遍历完毕后，数据被move
     for i in &v{
          println!("{:}",i);
     }

     println!("{:?}", v);
     // 若想在遍历的时候同时修改,需要使用mut
     // 若遍历的元素是&v, 则i为&T
     // 若遍历的元素是v, 则i 为T
     // 若遍历的元素是&mut v, 则i 为&mut T
     for i in &mut v {
          *i = 0;
     }

     println!("{:?}", v);
}
```

HashMap
-------

``` {.rust}
fn test_hashmap(){
     // 使用zip来创建
     let vec1 = vec!["haha"];
     let vec2 = vec!["hwihwi"];
     // 但是需要指名HashMap的类型,使用下划线即可，rust会通过vec推断出类型
     let mut map :HashMap<_,_>= vec1.iter().zip(vec2.iter()).collect();
     // 新建
     let mut map:HashMap<&str,&str> = HashMap::new();
     // let mut map = HashMap::with_capacity(5);
     // 插入数据
     // 注意插入数据是k,v，但是获取数据时使用的是&k
     map.insert("haha","hwihwi");

     println!("{:?}",map);
     // 使用get来获取值, 注意get中传入的参数是引用类型，所以需要加上&符号
     let v = map.get("haha");
     println!("{:?}",v);

     // 遍历, 注意此时的key,value也是引用类型
     for (key, value) in &map {

     }
     // 可以使用entry来获取键值对，如果发现无该键值对，可以使用or_insert来为其添加默认值,or_insert返回V的可变引用
     let v =  map.entry("haha").or_insert("huhu");
     *v = "lala"; // 可以使用or_insert返回的可变引用来修改map内部的值
     println!("{:?}", map);
     // 修改对应的值
     let e = map.insert("haha","hqhq");
}
```

错误
====

1.  问号运算符，如果结果是Err，则提前返回，否则继续执行
2.  问号运算符返回的是Result\<\>类型，所以?无法在main函数中运行

cargo
=====

cargo为项目管理工具，类比java的maven,c++的cmake等。 cargo的默认规则:

-   cargo.toml和cargo.lock文件总是位于项目根目录下。
-   源代码位于src目录下。
-   默认的库入口文件是src/lib.rs。
-   默认的可执行程序入口文件是src/main.rs。
-   其他可选的可执行文件位于src/bin/\*.rs(这里每一个rs文件均对应一个可执行文件)。
-   外部测试源代码文件位于tests目录下。
-   示例程序源代码文件位于examples。
-   基准测试源代码文件位于benches目录下。

cargo.toml和cargo.lock
----------------------

开发只需关心cargo.toml不需要关心cargo.lock,cargo.toml中是项目依赖的信息及版本，可对该文件进行自定义的更改。

IO
==

``` {.rust}
// 读取命令行参数  use std::env;
let args:Vec<String> = env::args().collect();
// 读取无效的Unicode字符 
let args:Vec<String> = env::args_os().collect();

// 读取文件，返回String内容 use std::fs;
let contents = fs.read_to_string(filename);

```

宏
==

-   实现宏有两种方式，1使用标准库macro~rules~!实现,2通过提供编译器扩展实现。

``` {.rust}
// 1. macro_rules
macro_rules! hashmap{
    // 第一个括号表示输入参数方式
    // +表示一个或者多个，*表示0个或多个
    ($($key: expr => $val: expr),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $val);)*  // 也可通过*符号扩展
            map
        }
    }
}

// 调用
hashmap!['A'=>1,'B'=>2];

// 实现println函数
macro_rules! myPrintln{
    ($format:expr, $input:expr)=>{
        println!($format, $input);
    }
}
// 调用
myPrintln!("{:?}",someVal);

// 定义函数
```

demo
====

编写minigrep
------------

代码目录

> .<br>
> ├── Cargo.lock<br>
> ├── Cargo.toml<br>
> ├── minigrep.iml<br>
> ├── poem.txt<br>
> ├── src<br>
> │   ├── lib.rs<br>
> │   └── main.rs<br>

lib.rs

``` {.rust}
use std::{env,fs, process, error::Error};

pub struct Config {
    query : String,
    fileName: String,
}

// Box 和dyn Error 配合使用的原因是，dyn Error是动态的，不明确大小的，而Box会将这种动态的数据放在堆上，符合Box的使用思路
pub fn run(config :Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.fileName)?;

    for line in search(&config.query,&contents) {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in contents.lines(){
        if line.contains(query) {
            result.push(line);
        }
    }
    result

    // 使用iter的写法
    /// contents.lines()
    ///    .filter(|line|line.contains(query))
    ///    .collect()
}

impl Config {
    pub fn new(args:&[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("not enough params");
        }
        let query = args[1].clone();
        let fileName = args[2].clone(); // 不使用clone,则为引用

        Ok(Config{
            query,
            fileName,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
```

main.rs

``` {.rust}
use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("config new runs err : {}", err);
        process::exit(1);
    });

    // 使用if let和unwrap_or_else的区别是，unwrap需要取到Result中的值, if let不需要unwrap(即取值)
    if let Err(err) = minigrep::run(config) {
        eprintln!("run error : {}", err);
        process::exit(1);
    }
}
```

use的使用
---------

``` {.rust}
// 复杂的用法
use {
    hyper::{
        Body, Client, Request, Response, Server, Uri,
        service::service_fn,
        rt::run,
    },
    futures::{
        compat::TokioDefaultSpawner,
        future::{FutureExt, TryFutureExt},
    },
    std::net::SocketAddr,
    tokio::await,
};
```

Programinng rust
================

-   rustdoc 用于编译文档
-   cargo创建项目，默认会创建.git文件，你可以使用 --vcs none 来跳过
-   一个rust包无论是lib还是bin都被称为crate
-   rust中的char是unicode编码的32位的
-   rust中使用u8类型来读取字节(byte)
-   b\'A\'是ASCII编码的字符,用u8表示就是65u8
-   box使得变量分配在堆中，且逃出scope后被释放，除非其被move
-   vec和数组的维一区别就是，vec的底层包含一个数组用于存放数据，同时有两个变量，一个是数据长度，一个是数据容量。在定义一个vec时，本体数据会被保存在heap里。而数组中的数据则直接在stack中分配。
-   数组的写法是\[T;n\],其中n是个数，而不加个数，则是slice,即\[T\]为slice,如\[u8\],slice通常不会被直接保存在一个变量中，或者直接被传递给一个函数，即你不会看见这种写法：
    `let a:[u8] = xxx` ，也不会这种写法: `func test(a [u8])`
    通常使用slice的时候都是通过 `&` 来使用，即借用或者是引用。
-   slice是一个胖指针，他有两个数值，一个是指向数据的头数据，一个是数据的长度。数组或者vec都可以被当作slice来使用.所以通常一个函数要对一个数组或者vec操作的时候，都会使用slice来进行声明。
-   r开头的string内的字符不会被转义，如 `r"\haha"`
    而b开头的string则是byte string
-   可以认为String是Vec\<u8\>, &str是&\[u8\],
    &str是一个胖指针，包含了数据的地址和数据长度。
-   String或&str的len()返回的是byte的大小，而chars().count()则是字符的大小。一个字面的字符串其实就是str.
-   一个字面的字符串是一个 str，注意不是&str,比如 `let a = "abc"`
    因为\"abc\"是str,那么a的类型就是str，而使用a的时候应该用&a来使用。
-   很重要!
    如果一个变量被赋值给了另一个变量，那么他的所有权就会被move,一定要记清这个概念！
-   基础类型都是Copy型的，在赋值的时候都是做copy操作(栈中)，若一个struct内的数据都是基础类型，只有当你derive了copy和clone,这个struct才是copy类型。
-   rc是reference count的缩写，而arc则是atomic reference
    count的缩写，从名字就能看出来，一个是线程不安全的，一个是线程安全的。
-   在使用Rc或者Arc的时候，必须通过clone方法来显性的复制相应的变量。Rc和Arc是智能指针，如Rc\<String\>那么你就可以对Rc使用String的任何方法。被Rc引用的值是不可变的，如果你要通过Rc改变内部值，则会报错。Rc中的值是不可变的一个原因是避免了循环引用，因为创建Rc的时候，其内容不可变，就导致他不会被修改为指向另一个指向自己的Rc上，所以避免了循环引用。但是rust中有内部可变性，所以也是可以通过一定的手段来达成循环引用，导致内存泄漏。
-   在使用for对一个map操作的时候，如果map是一个引用，那么for中的key,value同样也是引用，而若map是一个值(非ref)，那么for中的key,value也是值(会被move).而若map是一个可变引用，那么for中的value同样也是可变的。
-   在使用.操作符的时候，你不需要加上(\*)符号来解引用，他会进行隐式的解引用，同样的，.操作符也会隐式的进行借用，如有一个v类型是vec,那么
    v.sort和(&v).sort是一样的。
-   rust中的引用可以进行嵌套，如&&&&r或&&&r，他们最指向的都是同一个值，使用.操作符来操作的时候，也会自动的进行解引用到最终的那个值。如果使用双等号判断是否相等时，是判断的r值是否相等，如果想要判断两个引用是否指向同一个内存，那么可以使用std::ptr::eq方法。
-   rust中的引用不会是空，也没有默认值，一个引用只有在初始化或者被赋值的时候才能被使用。

心得
====

-   生命周期：一个函数中的生命周期相同的泛型表示这些参数具有相同的生命周期。相同并不是严格意义的相同，而是rust中有scope的概念，在一个变量进入一个新的scope后，就拥有与这个scope相同的生命周期了。所以如果b\>a，那么b在进入a的scope后，就和a一样了。而函数中的生命周期泛型，表示的返回值与输入值之间的联系，如果有了联系，就可以知道返回值在什么时候销毁，而如果两个不同的参数有相同的生命周期a,那么返回值的生命周期就是这两个生命周期相交的部分，一般来说b\>a时，b一般是包含a的，而不大可能存在b与a相交，所以b\>a时，返回的生命周期就是a。
