方式
====

``` {.go}
inputUrl := "http://localhost:123/haha?name=lala"
up, err := url.Parse(inputUrl)
checkErr(err)
up.Scheme()
up.Host()
up.Path()

// 待补充
```
