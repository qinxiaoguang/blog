用户输入
========

fmt.Scanxx
----------

### fmt.Scanln

``` {.go}
var s1,s2 string
fmt.Scanln(&s1,&s2)
```

以上是读取一行，以空格分隔，直到遇到非空字符。所以如果代码是这样的：

``` {.go}
var s1,s2 string
fmt.Scanln(&s1)
fmt.Scanln(&s2)
fmt.Printnln(s1,s2)
```

而输入了 `abcd efg` 那么输出将会是 `abcd fg`
,因为第二个单词的第一个字符被当作换行处理。但是如果输入 `abcd\n`
，那么第二个字符串将会读取不到，因为是按行来读取的，遇到换行直接就读完了。

### fmt.Scanf()

`fmt.Scanf` 和C语言中的scanf用法一样: `fmt.Scanf("%s%s",&s1,&s2)`

### fmt.Scan()

`fmt.Scan()` 不处理换行符，直到传入的参数都读取完毕。如
`fmt.Scan(&s1,&2)` ,读取的两个字符串之间可以有换行符。

fmt.Sscanxxx
------------

遇到S开头的scan函数，其处理的都是字符串，不会处理用户输入。 如:

``` {.go}
input := "abcd efg"
fmt.Sscan(input,&s1,&s2)
fmt.Sscanln(input,&s1,&s2)
fmt.Sscanf(input,"%s%s",&s1,&s2)
```

bufio
-----

利用bufio也可以读取数据：

### bufio.NewScanner

``` {.go}
inputScanner := bufio.NewScanner(os.Stdin)
inputScanner.Scan()
fmt.Println(inputScanner.Text())
```

其中inputScanner.Scan()会将读取的一行数据保存在内部的缓冲区中，使用inputScanner.Text()可以将其读取出来，注意Scan不会将换行符读进去，并且类似
`\n` 这种字符，他又不会去转义。

### bufio.NewReader

``` {.go}
inputReader := bufio.NewReader(os.Stdin)
input,err := inputReader.ReadString(',')
```

但是注意，这个函数和inputScanner不一样，这个是直接返回读取的数据以及error，其中参数表示，读取到对应的字符就停止，并且返回的数据包含该字符。所以如果输入是
`abcd,` ，则input的值就是 `abcd,` 不止ReadString可以读取， `ReadBytes`
也可以读取，不过 `ReadBytes`
返回的是byte类型的切片。byte类型的切片又和string可以互转，所以基本上是一样的。

文件读写
========

os.Open和bufio
--------------

`os.Open("input.txt")` 用于打开一个文件，文件类型一般是 `os.*File`
类型，而 `os.Stdin` 也是 `os.*File` 类型，所以对于 `bufio.NewScanner()`
或 `bufio.NewReader`
也可以传入一个文件。所以文件读取最简单的思路就是通过 `os.Open` 和
`bufio` 包实现:

``` {.go}
file,err := os.Open("test.go")
if err != nil{
    fmt.Println("读取文件失败")
}
inputScan := bufio.NewScanner(file)
for inputScan.Scan() {
    fmt.Println(inputScan.Text())
}
```

当然也可以通过 `bufio.NewReader(file)`
来实现，但是注意，这种方法实现的需要判断 `err`
，如果读取到文件结尾，将会返回io.EOF，所以读取整个文件的方式是：

``` {.go}
file,err := os.Open("test.go")
if err != nil{
    fmt.Println("读取文件失败")
}
inputReader := bufio.NewReader(file)
for {
    line,err := inputReader.ReadString('\n')
    fmt.Printf("%s",line)
    if err == io.EOF{
        return
    }
}
```

注意返回io.EOF的时候，line变量里也会有值。

ioutil
------

利用ioutil.ReadXxx的函数可以读取文件内容或用户输入,如:

``` {.go}
file,err := os.Open("test.go")
defer file.Close()
if err != nil{
    fmt.Println("读取错误")
}
bytes,err := ioutil.ReadAll(file)
if err != nil{
    fmt.Println("读取又错误")
}
fmt.Println(string(bytes))
```

ioutil.ReadAll传入的是io.Reader，所以对于os.Open，其刚好返回io.Reader，所以是可以的。
同样也可以使用 `ioutil.ReadFile` 方法，该方法其实就是对上述方法的封装:

``` {.go}
bytes,err := ioutil.ReadFile("test.go")
if err != nil{
    fmt.Println("读取错误")
}
fmt.Println(string(bytes))
```

缓冲读取
--------

``` {.go}
f,err := os.Open("test.go")
if err != nil{
    fmt.Println("错误")
}
fefer f.Close()
inputReader := bufio.NewReader(f)
buf := make([]byte,1024)
n,err := inputReader.Read(buf)
for n!=0 && err ==nil{
    fmt.Println(string(buf))
    n,err = inputReader.Read(buf)
}
```

其中inpuReader.Read方法传入的就是缓冲的参数，该函数返回的是n表示读入的字节个数。注意，一定要融会贯通，这样读取的方式不仅仅对文件使用，同样对屏幕输入适用。

fmt.Fscanxxx
------------

同样fmt里包含F开头的一些读取函数，与 `fmt.Sscanln` `fmt.Sscan`
`fmt.Sscanf` 是类似的，不过第一个参数从传入字符串，变成了传入文件。

文件写入
--------

最简单的是通过fmt.Fprintf将一个字符串写入:
`fmt.Fprintf(file,"some data\n")` 或 `fmt.Fprintln(file,"some data")`

``` {.go}
f,_ := os.Open("test.go")
_,err := fmt.Fprintln(f,"someData")
fmt.Println(err)
```

这种方式要注意权限，如果没有权限就会写入不成功 另一种方法是通过
`os.OpenFile` 来打开文件，并且赋予打开的方式，如:

``` {.go}
outputFile,err := os.OpenFile("test.go",os.O_WRONLY|os.O_CREATE,os.ModePerm)
if err != nil{
    fmt.Println("错误")
}
defer outputFile.Close()
outputFile.WriteString("哈哈哈，我终于写进来了")
```

需要注意的是，第二个参数是打开的模式，可以有只读模式，以及追加模式等，一般用的模式有：

``` {.go}
const (
    O_RDONLY int = syscall.O_RDONLY // open the file read-only.
    O_WRONLY int = syscall.O_WRONLY // open the file write-only.
    O_RDWR   int = syscall.O_RDWR   // open the file read-write.
    O_APPEND int = syscall.O_APPEND // append data to the file when writing.
    O_CREATE int = syscall.O_CREAT  // create a new file if none exists.
    O_EXCL   int = syscall.O_EXCL   // used with O_CREATE, file must not exist
    O_SYNC   int = syscall.O_SYNC   // open for synchronous I/O.
    O_TRUNC  int = syscall.O_TRUNC  // if possible, truncate file when opened.
)
```

在写入文件的时候，第三个参数必须是0666，而对于读的时候可以忽略。
如果写入的内容过多，可以通过bufio来创建一个有缓冲的写入器:

``` {.go}
bufio.NewWriter(outputFile)
```

文件覆盖
--------

文件覆盖在打开文件的时候，一定要加上os.O~TRUNC~,或者在open file后调用
file.Trunscate(0),否则达不到文件覆盖的效果

``` {.go}
outputFile,err := os.OpenFile("test.go",os.O_WRONLY|os.O_CREATE|os_O_TRUNC,0666)
// or outputFile.Trunscate(0)
if err != nil{
    fmt.Println("错误")
}
defer outputFile.Close()
outputFile.WriteString("哈哈哈，我覆盖了文件")
```

文件拷贝
========

`io.Copy(target,source)`
但是注意source文件可以通过os.Open读取，而target文件必须使用os.OpenFile来读取，并赋予os.O~WRONLY标志或者其他可写标志~。

命令行读取
==========

命令行读取可以通过 `os.Args`
来获取命令的参数，同样可以通过flag包来设置参数。 比如输入:
`./goTest hahaha lalal`
那么os.Args中存放的就是\[./goTest,hahaha,lalal\]，可以通过os.Args\[1\]这种方式来取出。

但是如果我想要 `./goTest -name Mike`
这种方式来取数据怎么办呢，方法就是通过flag包。 比如:

``` {.go}
name := flag.String("name","","输入你的名字~")
flag.Parse()
fmt.Println(*name)
```

注意name是一个指针类型，通过flag.Parse()方法可以解析命令行参数，通过
`*name` 将输入取出， `flag.String`
的第一个参数表示命令行中的参数名，如输入 `./goTest -name "hahah"` ，其中
`-name`
就是对应第一个参数，第二个参数是name的默认值，如果没有输入，name的值就是这个默认值，第三个参数是说明，一般只有在
`help` 中出现。

当然不仅仅有 `flag.String` 还有 `flag.Bool` 等方法。 `falg.StringVar` 跟
`flag.String` 一样，只不过 `flag.StringVar`
没有返回值，而是将name指针传给了该方法:
`flag.StringVar(&name,"name","","输入你的名字~")`
