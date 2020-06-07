简介
====

pssh是parallel-ssh,用于针对ssh进行的批处理工具。

安装
====

google已经关闭了这个项目，所以安装的话，来这里下载：https://clsn.io/files/pssh/
百度下载地址：https://pan.baidu.com/s/1co3Hwoc0yI4LAKvXoXPzfg
（提取密码: d2jy）

(摘自：https://www.cnblogs.com/kevingrace/p/6378719.html)

安装方法:解压完后，运行 `python setup.py install`
安装完后可能会遇到Manager或Version模块不存在，把解压的目录里的build/lib/psslib复制到你的python环境根目录里，或者自行查看
pssh 命令去哪个目录找的psslib包。而Version不存在，则把 `import Version`
替换为 `from . import Version` 即可。

使用
====

| -h 执行命令的远程主机列表文件
| -H user\@ip:port 文件内容格式\[user@\]host\[:port\]
| -l 远程机器的用户名
| -p 一次最大允许多少连接
| -o 输出内容重定向到一个文件
| -e 执行错误重定向到一个文件
| -t 设置命令执行的超时时间
| -A
  提示输入密码并且把密码传递给ssh（注意这个参数添加后只是提示作用，随便输入或者不输入直接回车都可以）
| -O 设置ssh参数的具体配置，参照ssh~config配置文件~
| -x 传递多个SSH 命令，多个命令用空格分开，用引号括起来
| -X 同-x 但是一次只能传递一个命令
| -i 显示标准输出和标准错误在每台host执行完毕后
| -I 读取每个输入命令，并传递给ssh进程 允许命令脚本传送到标准输入
