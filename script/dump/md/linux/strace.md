概述
====

strace命令是用于监控用户进程和内核的交互，如系统调用，信号传递，进程状态变更等。因自己的服务模块出现了socket泄漏，看了一些资料，大多数都是使用strace来进行的分析，所以需要xiao习一下这个命令。

参数
====

-   `-tt` 在每行输出的前面，显示毫秒级别的时间
-   `-T` 显示每次系统调用所花费的时间
-   `-v` 对于某些相关调用，把完整的环境变量，文件stat结构等打出来。
-   `-f` 跟踪目标进程，以及目标进程创建的所有子进程
-   `-e` 控制要跟踪的事件和跟踪行为,比如指定要跟踪的系统调用名称
-   `-o` 把strace的输出单独写到指定的文件
-   `-s`
    当系统调用的某个参数是字符串时，最多输出指定长度的内容，默认是32个字节
-   `-p` 指定要跟踪的进程pid, 要同时跟踪多个pid, 重复多次-p选项即可。

其中-e相关的参数：

-   `-e trace=file` :跟踪和文件访问相关的调用(参数中有文件名)
-   `-e trace=process` :和进程管理相关的调用，比如fork/exec/exit~group~
-   `-e trace=network` :和网络通信相关的调用，比如socket/sendto/connect
-   `-e trace=signal` :信号发送和处理相关，比如kill/sigaction
-   `-e trace=desc` :和文件描述符相关，比如write/read/select/epoll等
-   `-e trace=ipc` :进程见同学相关，比如shmget等

举例
====

服务启动时即追踪其系统调用: `strace ./startServer.sh` 追踪网络通信:
`strace -e trace=network ./startServer.sh` 追踪相关进程调用:
`strace -p ${PID}` 追踪详细内容:
`strace -tt -T -f -o ./tmp.txt ./startServer.sh`
