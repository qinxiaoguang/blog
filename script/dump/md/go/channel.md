TIPs
====

-   `runtime.GOMAXPROCS(numCores)` 用来设置使用的核心数量
-   `runtime.Goexit()` 终止协程
-   channel定义的方式是: `var identifiier chan datatype` ,注意
    `chan func()` 这种也是channel类型，不过他的通道的内容是函数而已。
-   channel是引用类型，所以可以在函数中传来传去
-   同一个channel，可以传入元素值，也可以传入指针
-   channel必须使用make()来创建，如果 `var ch chan int`
    创建，则初始值是nil，无法使用
-   channel可以传入空字节，如 `type Empty interface{}` 可以创建
    `ch := make(chan Empty)`
-   channel默认是无缓冲的，也就是说在main中执行 ch\<- 1
    的时候会阻塞，同样执行 \<-ch的时候也会阻塞。
-   创建带有缓冲的channel方式是: `make(chan int,100)`
-   通道也可以用在range上 `for v:= range ch{}`
-   `chan<-int` 表示只接受内容的通道不允许发送，这种通道无法关闭，
    `<-chan int` 表示只发送的通道
-   可以将双向通道转换为单向通道
-   关闭通道的方式是 `close(ch)` ,在关闭之后，可以通过 `v,ok := <- ch`
    来检测是否被阻塞，如果ok为false,表示ch被关闭。
-   如果 `v,ok := <-ch` 来检测通道是否关闭有点繁琐，有更简单的方式是使用
    for: `for v:=range ch{}`
-   如果channel被关闭，不能重复再关闭，也不能向channel中写数据，但是channel中剩余的数据还是可以读的。

``` {.go}
select {
case u:= <- ch1:
        ...
case v:= <- ch2:
        ...
        ...
default: // no value ready to be received
        ...
}
```

-   select是轮询机制，如果有default，那么每次select都会瞬间执行其中一个分支，如果case都没有成功，就会执行default，否则执行case中的某一个。
-   而如果没有default，那么case如果都阻塞，select也会发生阻塞，直到有一个case成功。

可以通过 `time.Tick(time.Second*2)`
来创建定时器，其中的通道会隔一段时间向通道内发送内容，如：

``` {.go}
ticker := time.Tick(time.Second*2)
for{
    <-ticker
    fmt.Println("hah")
}
```

上述代码会隔两秒打印一次 `hah` 也可以通过
`time.NewTicker(time.Second*2)` 创建定时器，不过返回的是一个结构体，如:

``` {.go}
ticker := time.NewTicker(time.Second*2)
for{
    <-ticker.C
    fmt.Println("hah")
}
```

第二种写法调用的时候，需要指明其中通道，这种方式的通道名为 `C`
,所以获取该定时器的方式就是 `ticker.C`

`time.After()`
只会向通道内发送一次内容，将select和这个结合，可以实现超时的功能：

``` {.go}
after := time.After(time.Second*5)
ch := make(chan int)
select{
case <- ch:
    fmt.Println("来啦")
case <-after:
    fmt.Println("超时拉")
}
```

同样有更简便的写法:

``` {.go}
ch := make(chan int)
select{
case <- ch:
    fmt.Println("来啦")
case <-time.After(5e9):
    fmt.Println("超时拉")
}
```

`1e9` 就是1s
