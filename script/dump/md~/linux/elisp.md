相关教程
========

1.  《GNU emacs lisp编程入门》
2.  <http://ergoemacs.org/emacs/elisp_basics.html>

看了第二个李杀教程，我觉得笔记记录的没必要了。

简介
====

-   lisp:list processing,
    列表处理，括号是列表的边界，有时候个列表用一个单引号或 `'` 开头。
-   `'(rose violet daisy)`
-   `(+ 2 5)`
-   `(expt 2 3)` 表示次方
-   列表通俗来说就是函数调用。
-   组成列表的元素为原子，如 `+` 即为原子。
-   原子和列表的书面表示被称作 `符号表达式` 或 `s-表达式`
-   双引号中的文本，不论是句子还是一个段落，都是一个原子。被称作为string,
-   列表中空格无关紧要，换行也被当做空格处理，多余的换行和列表只是更易于阅读。
-   `‘`
    单引号在一个列表前，则该列表不被执行，被当成引用,执行时原样输出。
-   在emacs中可以使用 `C-x C-e`
    对当前光标前的列表进行执行，并将结果打印在回显区中，使用
    `C-u C-x C-e` 打印结果则显示在当前光标后。
-   处理textinfo 的所有函数都以 `textinfo-` 开头，阅读电子邮件的函数都以
    `rmail-` 开头。
-   `.elc` 文件是字节编码文件，可以在 `emacs/lisp` 看到这两种文件
-   `C-x C-e` 实际上就是执行的 `eval-last-sexp` 即对最近一个s-表达式求值

变量
====

-   在文件中键入fill-column，并使用 `C-x C-e` 就可以查看其被赋的值
-   对一个变量求值时，周围没有括号，是因为不想把其当做函数来使用
-   可以使用 `set setq let` 等函数对变量赋值

使用set函数赋值
---------------

-   使用set赋值时，需要在变量前使用单引号，否则会被当做求值进行处理，如：
    `(set 'flowers '(heihei))` 而不能写为 `(set flower '(heihei))`

使用setq函数
------------

-   和set函数的区别是，setq要赋值的变量不需要单引号。同时，setq可以给多个变量赋值，如:
    `(setq flower '(heihei))` `(setq flower "1" flower2 "2")`
-   set和setq都可以将列表赋值给某个变量，如上述的
    `(setq flower '(heihei))` 注意列表前需要单引号，否则被求值。

使用let函数
-----------

和haskell中的let ..in 类似。

-   let创建局部变量，且一次可创建多个变量
-   let表达式第一部分是let符号，第二部分是一个列表，为变量列表，该列表每个元素是个符号或是一个两元素的列表，第三部分是let表达式的主体，这个主体有一个或多个列表组成,
    其实就是要执行的命令。
-   第二部分如 `(var1 (var2 3))` 表示var1初始值为nil,var2初始值为3

求值
----

-   每次在emacs中做一些编辑命令，如移动光标，都是在对表达式求值，如插入一个字符的时候，实际调用的是
    `self-insert-command`

缓冲区
------

使用emacs对一个文件进行编辑的时候，通常是对缓冲区编辑，不会影响文件，如果保存了编辑内容，才会把缓冲区内容输入到文件中。

-   `(buffer-name)` 表示当前缓冲区名，通常是当前文件名
-   `(buffer-file-name)`
    表示与当前缓冲区关联的文件的文件名，通常是当前文件的完整文件名
-   `(current-buffer)` 指向当前缓冲区，注意与 `(buffer-name)` 的区别，
    `(buffer-name)` 只是缓冲区名字
-   `(other-buffer)` 返回最近使用过的缓冲区。
-   `(switch-to-buffer)` 是切换buffer的命令，执行 `C-x b`
    的时候实际调用的就是这条命令，如执行 `C-x b` 并输入 `*scratch*`
    时，则切换到 `*scratch*` 缓冲区，如果 `(other-buffer)` 只返回
    `*scratch*` ,则效果和 `(switch-to-buffer (other-buffer))` 一样。
-   `(set-buffer)`
    屏幕显示的缓冲区并不改变，它将计算机的注意力切换到另外一个缓冲区
-   `(buffer-size)` ：当前缓冲区的大小，或总字符个数
-   `(point)` 当前光标所在的位置
-   `(point-min)` 返回光标可能所在位置的最小值， `(point-max)`
    返回光标可能所在位置的最大值。

编写函数
========

-   使用 `defun`
    定义函数,包含几个部分(函数名、参数名、文档描述、交互信息(可选)、主体)等。

``` {.commonlisp org-language="lisp"}
(defun func-name(arguments ...)                                               
    "operation-documention..."
    (interactive argument-passing-info);optional
    body...)
```

如定义乘7的函数:

``` {.commonlisp org-language="lisp"}
(defun multiply-by-seven (number)
    "multiply NUMBER by seven"
    (* 7 number))
```

-   键入 `C-h f`
    输入函数名所查看到的内容就是\"operation-documention\"的内容
-   永久安装函数：1. 在.emacs文件中编写，2. load函数 3.
    在site-init.el文件中编写。

交互函数
--------

-   关键字 `(interactive)`
-   用户键入 `M-x`
    和函数名就可以激活一个交互函数，或者键入绑定的键序列(如C-n绑定next-line)
-   `(interactive "p")`
    中\"p\"告诉emacs将你正在键入的C-u后加上一个数字或Meta加一个数字，将这个数字传递给函数。如
    `C-u (参数) M-x func-name` 或 `M-(参数) M-x func-name`
-   interactive后边可有很多字符，如字符"r"表示将当前光标点的开始值和结束值作为函数的两个参量。"B"则为缓冲区的名字作为参量,
    interactive在传多个参量的时候，之间用\"\n\"分割，如\"B\nr\"传入了\"B\"和\"r\"
-   interactive也可以没有参量，如果一个函数中设置了(interactive)则表示该函数可进行交互，就能通过
    `M-x` 来进行调用。

if
==

第一种无else的表达式，格式为:

``` {.commonlisp org-language="lisp"}
(if true-or-false
    true-action)
```

如:

``` {.commonlisp org-language="lisp"}
(if (> 5 4)
    (message "5 is greater than 4"))
```

第二种有else的表达式如:

``` {.commonlisp org-language="lisp"}
(if true-or-false
    true-action
  false-action)
```

其中else-part紧跟then-part后，但缩进比then-part少，如：

``` {.commonlisp org-language="lisp"}
(if (> 5 4)
    (message "5 is bigger than 4")
  (message "5 is smaller than 4"))
```

真假
====

lisp中nil即为假，nil一方面表示假，一方面也表示空列表，可以将nil写为()或nil。非nil即为真。
如

``` {.commonlisp org-language="lisp"}
(defun testTrue ()
  ""
  (message "ha"))
(if (testTrue)
    (message "5 is bigger than 4")
  (message "5 is smaller than 4"))
```

会输出\"5 is bigger than 4\",因为testTrue返回的有值。 再如:

``` {.commonlisp org-language="lisp"}
(if 4
    'true
  'false)
(if nil
    'true
  'false)
```

save-excursion函数
==================

在对其主体进行求值时候，会保存当前缓冲区中的标记(mark)和位点(point)以及当前缓冲区，在求值完成后恢复原来位点和标记
值以及缓冲区。lisp中的某些函数在操作的时候，经常会移动位点(point),如count-lines-region.
表达式模板:

``` {.commonlisp org-language="lisp"}
(save-excursion 
  body...)
```

函数体是一个或者多个表达式，被依次求值，最后一个表达式的返回值被当做结果。
在elisp中，save-excursion表达式经常出现在let表达式主体中，如：

``` {.commonlisp org-language="lisp"}
(let varlist
  (save-excursion
    body...))
```

while
=====

模板

``` {.commonlisp org-language="lisp"}
(while true-or-false
  body...)
```

cond
====

类似switch，如:

``` {.commonlisp org-language="lisp"}
(let ((number 1))
  (cond ((< number 0) 0)
    ((> number 0) 1)))
```

与缓冲区有关的函数
==================

-   `beginning-of-buffer`
    是将光标移动到缓冲区的开始位置，并在之前的位置放入标记(push-mark)。与
    `M-<` 绑定
-   `end-of-buffer`
    是将光标移动到缓冲区的结束位置，并在之前的位置放入标记(push-mark)。与
    `M->` 绑定
-   `mark-whole-buffer`
    在当前位点及末尾点放置标记，并跳转到缓冲区开头处。
-   `append-to-buffer`
    从当前缓冲区中拷贝一个域(即缓冲区中介于位点和标记之间的区域)

基本函数
========

-   `car` 返回列表中第一个元素，如 `(car '(rose flower))` 返回rose
-   `cdr` 返回列表中除了第一个元素的其余部分，如
    `(cdr '(rose flower hei))` 返回(flower hei)
-   `cons` 构造列表，如 `(cons 'hei '(haha  hengheng))` 返回
    `(hei haha hengheng)`
    ,其中第一个参量是被插入的元素么，第二个参量必须是列表。
-   `length` 查看列表中有多少个元素 `(length '(haha hei)` 返回2
-   `nthcdr` 除了头几个元素的剩余的所有元素，nthcdr表示执行n次cdr，如
    `(nthcdr 2 '(1 2 3 4 5))` 返回 `(3 4 5)`
-   `setcar` 是将一个列表的初始元素设置为新的值，如
    `(setq animal '(dog cat))` `(setcar animal 'bee)` 则animal的结果为
    `(bee cat)`
-   `setcdr` 是将列表中除了第一个元素的其余值进行替换。

配置文件
========

`site-start.el` -\> `.emacs` -\> `default.el`

绑定键
======

-   `(global-set-key "\C-cw" 'compare-windows)` 表示将 `compare-windows`
    函数绑定给 `C-c w`
-   取消的方式为 `(global-unset-key "\C-cw")`
-   `(define-key )` ????

加载文件
========

-   从其他文件中加载对应的方法，如 `(load "~/emacs/kfill")`
    即加载文件\~/emacs/kfill.el(c)，可类比shell中的source
-   autoload是自动加载，emacs启动的时候，不会加载其内容，而是等到第一次使用其内部的方法的时候才会加载。该函数包含五个参量，第一个是自动加载的方法名，第二个参量是被加载的文件名，第三个是为这个函数编写的文档，第四个参量告知该函数是否能被交互调用，第五个告诉对象是什么类型的。
