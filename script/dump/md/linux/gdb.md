概述
====

gdb是一个调试工具，重点是调试工具，而不是c/c++的相关工具，其是由GNU开源的工具.
参考:<https://blog.csdn.net/liigo/article/details/582231>

调试命令
========

  命令                             解释                                                                                                                    示例
  -------------------------------- ----------------------------------------------------------------------------------------------------------------------- -----------------------------------
  file \<filename\>                加载被调试的可执行文件                                                                                                  (gdb) file somefile
  r                                run的简写，运行被调试的程序                                                                                             (gdb) r
  c                                continue的简写，继续执行程序，直到下一个断点                                                                            (gdb) c
  b \<行号\>                       breakpoint 设置断点。 b \<行号\> 在指定行号加断点，b \<函数名称\>/ \*\<函数名称\>/\*\<代码地址\>                        (gdb) b 8/main/\*main/\*0x804823c
  d                                delete breakpoint 删除断点                                                                                              (gdb) d
  s                                step into: 执行一行源程序代码，如果此行代码中有函数调用，则进入该函数。                                                 (gdb) s
  n                                step over: 执行一行源程序代码，此行代码中的函数也一并执行。                                                             (gdb) n
  si,ni                            与s/n命令相同，不同的是si/ni针对的是汇编指令，而s/n针对的是源程序                                                       (gdb) si/ni
  p \<变量名称\>                   print的简写，显示指定变量(临时变量或全局变量)的值                                                                       (gdb) p i
  display .../ undisplay\<编号\>   display,设置程序中断后欲显示的数据及格式，如display /i \$pc可查看中断后的下一条汇编指令，undisplay取消先前dispaly设置   (gdb) display /i \$pc
  i                                info的简写，用于显示各类信息                                                                                            (gdb) i r
  q                                quit的简写，退出gdb                                                                                                     (gdb) q
  help \<命令名称\>                帮助                                                                                                                    (gdb) help display
