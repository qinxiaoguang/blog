什么是core dump
===============

在使用半导体作为内存的材料前，人类是利用线圈当作内存的材料（发明者为王安），线圈就叫作
core ，用线圈做的内存就叫作 core memory。如今
，半导体工业澎勃发展，已经没有人用 core memory 了，不过，在许多情况下，
人们还是把记忆体叫作 core 。
即core为内存,dump意为将内存倾倒下来.其作用就是在程序出现各种异常或bug导致程序退出或终止,会产生一个core的文件,这个过程就是core
dump.该文件内容记录了程序在退出时内存的信息.

core相关配置
============

core文件大小
------------

`ulimit -c` ulimit命令可以查看linux系统的一些初始化的参数,其中
`ulimit -c` 是查看core size,
即core文件的可创建大小,如果是0,则表示无法core dump.

core文件生成位置及命名
----------------------

`/proc/sys/kernal/core_uses_pid`
用来控制core文件的文件名中是否添加pid作为扩展,如果为1则添加,为0不添加
`/proc/sys/kernal/core_pattern`
可用来控制core文件生成的位置和名字,如文件内容为
`/home/coresave %e %p %u %g %s %t`
则表示将core文件统一生成到/home/coresave文件中,其中参数表示如下:

-   \%p - insert pid into filename 添加pid
-   \%u - insert current uid into filename 添加当前uid
-   \%g - insert current gid into filename 添加当前gid
-   \%s - insert signal that caused the coredump into the filename
    添加导致产生core的信号
-   \%t - insert UNIX time that the coredump occurred into filename
    添加core文件生成时的unix时间
-   \%h - insert hostname where the coredump happened into filename
    添加主机名
-   \%e - insert coredumping executable name into filename 添加命令名

调试
====

使用gdb进行调试 `gdb [exe file] [core file]` 其中\[exe
file\]是程序执行目录,\[core file\] 是core文件目录.
在进入gdb后,where或者bt命令可以进行backtrace
