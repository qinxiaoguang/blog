引导
====

程序入口
--------

随便写一个main.go代码，之后使用
`go build -gcflags "-N -l" -o main main.go` 来编译 使用 `gdb test`
进入调试命令行，并使用 `(gdb) info files` 或者 `(gdb) i files`
来列出程序运行的相关文件，其中可以看到这一行

| Entry point: 0x104e8b0

这就是程序入口，使用 `(gdb) b *0x104e8b0`
给该位置打上断点，其就会提示断点到哪个文件，对应的文件就是程序的入口文件。
如本程序显示

| Breakpoint 1 at 0x104e8b0: file
  /usr/local/go/src/runtime/rt0~darwinamd64~.s, line 8.

所以入口文件为 `file /usr/local/go/src/runtime/rt0_darwin_amd64.s`
按照这个流程可以找出go程序的执行流程:

1.  汇编
2.  初始化runtime
3.  创建go协程

初始化
------

初始化的流程,可以在汇编代码中查看到流程大概如下

-   runtime.args() // 初始化参数，不重要
-   runtime.osinit() // 初始化cpu个数
-   runtime.schedinit() //
    重点在这，其会初始化内存，调度,垃圾回收等各种相关内容，其中GOMAXPROCS环境变量也是在这处理
-   runtime.main()
    //runtime.main和main~main是不同的~,runtime.main会启动后台监控，执行定期gc及调度，并执行runtime中的动态生成的init函数，及main程序中的init函数(main~init~),之后会执行main~main函数~，及main.go的main函数
-   runtime.mstart()

init的执行顺序为，先执行本包中import中的包中的init函数，这些函数的init执行顺序是按照file名的字母顺序执行，且同一个文件中的init函数，按照前后顺序执行。执行完毕后，会执行本代码所处的包文件中的init函数,注意init函数是可以存在多个的。
注意所有init的函数都是在同一个goroutine内执行的。

内存管理
--------

内存管理使用的TcMalloc,请查阅:<https://www.jianshu.com/p/11082b443ddf>
来了解什么是TcMalloc
