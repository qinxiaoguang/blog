flag操作
========

首先说明flag是干什么的。 我们都看到过在linux下这样执行的命令:

``` {.shell}
cmd -name value
```

那么我想在go执行的时候，也加上-name
value这样的命令，并且能够接受到，怎么做呢，方法就是使用flag包。
如下所示:

``` {.go}
func main() {
    var q = flag.Bool("name",false,"tips")  //创建name名称绑定的变量，注意，q是指针类型的，默认是false
  var p = flag.String("p","82220","port")
    flag.Parse()  //通过flag.Parse来解析命令行中的变量值，如果没有就使用默认值
    fmt.Println(*q) //打印该值
  fmt.Println(*q)
}
```

调用方式就是\`./main -name true\`
