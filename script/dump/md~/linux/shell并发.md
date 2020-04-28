基础知识
========

-   实现并行操作需要使用管道
-   `mkfifo fifotest` 可创建管道文件，使用 `echo "data" > fifotest`
    就可以向该管道文件添加数据，这个管道其实就是
    `cat tmp.txt | grep abc` 中的 `|` 符号.
-   管道存在的时候，如果没人读取管道中的数据，则管道会阻塞，而如果有人读取数据时，而此时该管道中无数据则也会阻塞，可以类比golang中的channel.
-   存管道中读取数据的方法是 `cat fifotest` 或其他命令 =read=等
-   若想管道不进行阻塞，可使用文件描述符。系统自带的文件描述符是0,1,2，分别代表stdin、stdout、stderr。
-   自己创建文件描述符可以创建3\~n-1, 其中n的值是 `ulimit -n` 代表的值。
-   使用 `exec 3<> fifotest`
    可将3这个文件描述符和fifotest这个管道进行关联，此时3这个文件描述符和fifotest的功能一模一样,但是向3这个文件描述符中写数据和取数据都不会造成阻塞。
-   关闭文件描述符的方法是 `exec 3>&-; exec3<&-;`
-   参考 <http://blog.sina.com.cn/s/blog_7099ca0b0100nby8.html>

实现
====

通过以下命令实现并行操作

``` {.shell}
#!/bin/bash
read -p "请输入并发数" N
exec 128>&-;exec 128<&-
# 创建管道并关联文件描述符128
[ -e /tmp/fifofile ] || mkfifo /tmp/fifofile
exec 128<> /tmp/fifofile
rm -rf /tmp/fifofile
# 向管道中push N个空行，表示N个并发
for((i=0;i<$N;i++))
do
    echo >&128
done
for((i=0; i<1000;i++))
do
    read -u128 # 取
    {
        echo "success"$i
        sleep 1
        echo >&128 # 归还
    }&
done
# 调用wait,等待后台进程运行完毕
wait
# 关闭文件描述符的写
exec 128>&-;
# 关闭文件描述符的读
exec 128<&-;
```
