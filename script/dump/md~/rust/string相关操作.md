String和&str的区别
==================

String内部是一个Vec\<u8\>,其中的内容是utf-8编码数据,且String内的数据是在堆上分配的。所有String中的len()方法返回的是字节数量，如果想找到第几个字符，可以通过\`chars().nth()\`等方法。
而str中的数据是一个常量。如 `let a = "hahaa"` 中的\"hahaa\"
就是一个常量，但注意a是&str类型。 从&str转换为String的方法:

``` {.rust}
1. String::from("aaa")
2. "aaa".to_string()
3. "a".to_owned() // to_string的底层方法就是调用的to_owned()
```

而String转&str: `&var` 或 `var.as_str()`
因rust中的deref,&String是可以被当作&str来使用的。所有string的通用参数类型基本都会选择&str.
因很多trait没有为&str实现，而是为String实现，如符号重载 `+` ,所有有
`String+&str -> String` 的操作，而没有 `&str + &str`
的操作。所以要实现String的加法运算，需要先将 `&str` 转换为 `String`

操作
====

split
-----

``` {.rust}
// 常规split
let ss = "/a/b/c"
let res = ss.split("/");
// 对split的结果进行遍历
for i in ss.split("/"){}
// 遍历结果获取下标
for (index, item) in pattern
    .split(URL_PATH_SPLIT)
    .collect::<Vec<&str>>()  // 注意这个地方，需要注视collect的类型，否则不知道你要collect什么类型。
    .iter()
    .enumerate(){

}
```

equal
-----

比较太恶心了，必须得解决一下。

1.  match
2.  equal
