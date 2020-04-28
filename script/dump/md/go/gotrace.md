概述
====

`go tool trace`
是一个用于诊断性能及问题的工具，其能将当前的go程序的运行信息通过ui来展示出来。
`go tool pprof`
也是一个性能诊断工具，但是其ui比较简陋,且其主要针对内存,cpu及协程等进行的分析。

trace使用方式
=============

简单举例:

``` {.go}
package main

import(
    "os"
    "fmt"
    "runtime/trace"
)

func main(){
    // 创建一个trace的文件
    f, err := os.Create("trace.out")
    checkErr(err)
    defer f.Close()
    // 开始trace
    err = trace.Start(f)
    checkErr(err)
    defer trace.Stop()

    fmt.Println("Hello,Trace")
}
```

之后只需要运行程序，程序会生成一个trace.out文件，之后运行
`go tool trace trace.out` 即可进行解析，在浏览器打开相应的地址即可。

pprof使用方式
=============

简单举例:

``` {.go}
// pprof 的init函数会将pprof里的一些handler注册到http.DefaultServeMux上
// 当不使用http.DefaultServeMux来提供http api时，可以查阅其init函数，自己注册handler
// 在main程序简单的引入该文件即可,并打开相关端口
// 一般来说该种方式是用来分析web程序的。
// 其他正常运行
import _ "net/http/pprof"

go func() {
    // 需要指定相关端口，能让浏览器访问到
    http.ListenAndServe("0.0.0.0:8080", nil)
}()

// 如果是非web程序，可以通过相关程序来手动生成prof文件，如cpu
f,_ := os.Create("cpu.prof")
pprof.StartCPUProfile(f)
defer pprof.StopCPUProfile()

//如heap:
f,_ := os.Create("heap.prof")
pprof.WriteHeapProfile(f)
```

运行完程序后，在浏览器中运行:<http://localhost:8080/debug/pprof/>
可以看到下边的很多数据，也可以将其中的profile下载下来。下载下来后，使用go的工具进行分析:
`go tool pprof exe profile`
其中exe为go程序的可执行文件，进入该指令下后，可输入top查看cpu占用前10的函数.而输入web则可通过浏览器查看UI样的内容。
其中生成的prof文件，也可通过以上命令进行分析。

参考
====

<https://making.pusher.com/go-tool-trace/>
<https://www.cnblogs.com/Leo_wl/p/7426618.htmlcd>
<https://studygolang.com/articles/9940>
