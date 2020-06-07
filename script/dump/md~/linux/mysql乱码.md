背景
====

mysql默认的某些编码格式不是utf-8，所以导致部分数据在访问的时候中文显示为??

解决方案
========

目前遇到的简单且有效的方式是在mysql中执行 `set names utf8;` 可以使用
`show variables like "%character%";` 来查看编码格式。
