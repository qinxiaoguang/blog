概述
====

在登录一些机器的时候使用ssh来进行登录，需要填写密码，但是这种情况能不能自动化呢。答案是可以，相应的工具就是expect.expect是一个工具，不是linux自带的，所有使用前需要进行安装，如通过
`brew` 来进行安装。如果不会的话，百度一下。

example
=======

``` {.shell}
#!/usr/bin/expect  # 要执行expect必须有此开头，表示使用expect进行执行，其类似sh
#set timeout 20 # 设置超时时间 如果没设置，那么expect在没有收到相应的字符的时候，就会根据超时时间退出。
spawn ssh root@192.168.43.131  # spawn会另起一个进程，并执行后边的指令,开进程后可使用expect/send与该进程进行交互。
expect "*password:"  # expect 如果出现了后边跟的字符，则会执行后续的指令，否则会有超时时间，并立即退出
send "123\r" # 类似人工输入命令，在expect后可以进行发送命令。
# expect "*#"
# 数组格式的expect
expect {
    "*password:*" {
        # do something
    }
    "*second*" {

    }
}
interact # 执行完毕后将该进程的控制权交给用户。如果没有，则执行完就会退出。
```

相关命令
========

输入输出
--------

使用 `puts` 进行输出:

``` {.shell}
puts stderr "Usage: $argv0 login passwaord.n "
puts "hello world"
puts stdout "1234"
```

获取参数
--------

使用\$arg0 \$arg1 ... 等来获取传入的参数, \$argc是参数的长度:

``` {.shell}
if {$argc < 2} {
    puts stdout "$argv0 err params\n"
    exit 1
}
```

设置变量值
----------

使用 `set` 命令:

``` {.shell}
set user [lindex $argv 0] # lindex表示取出某个数组中的值，如此处表示取出argv数组中的下标0的值。
set password [lindex $argv 1]
```

spawn
-----

开启一个进程来执行后边跟着的命令:

``` {.shell}
spawn ls -l
```

函数定义和调用
--------------

``` {.shell}
proc fnName {para1 para2} { 

}

fnName $argv0 $argv1 # 调用
```

循环
----

``` {.shell}
while (true) { 

}
```

if/switch
---------

``` {.shell}
switch -- $var { 

0 {

  } 
1 {

  }
}

if { $param == "haha" } { # 注意其中的括号

} else {

}
```

超时设置
--------

expect默认执行的超时时间是10s,可以使用 `set timeout -1`
来设置永不超时,而 `set timeout 10` 则10s超时

字符串操作
----------

字符串格式化:

``` {.shell}
set tmp [format "--haha %s" $other] // 给tmp赋format后的值
```
