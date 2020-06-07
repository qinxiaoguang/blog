time格式
========

`20060102150405`

-   2006 : year
-   01 : month
-   02 : day
-   15 : hour
-   04 : minute
-   05 : second

time转string
============

``` {.go}
time.Now().Format("2006-01-02 15:04") 
```

解析string
==========

``` {.go}
// 第一个参数是时间格式化叶样式，第二个参数是字符串的时间，第三个是时间区
time.ParseInLocation("2006-01-02T15:04:05Z", "2019-04-24T04:47:11Z", time.Local)
```
