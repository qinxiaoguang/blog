get
===

``` {.go}
func httpGet() {
    resp, err := http.Get("url")
    if err != nil {
        // handle error
    }

    defer resp.Body.Close()
    body, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        // handle error
    }

    fmt.Println(string(body))
}
```

post
====

一种是使用http.Post方式

``` {.go}
func httpPost() {
    resp, err := http.Post("http://www.01happy.com/demo/accept.php",
        "application/x-www-form-urlencoded",
        strings.NewReader("name=cjb"))
    if err != nil {
        fmt.Println(err)
    }

    defer resp.Body.Close()
    body, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        // handle error
    }

    fmt.Println(string(body))
}
```

传递json的话，将strings.NewReader()换成bytes.NewReader()，contentType换为\`application/json\`即可。
Tips：传递string使用这个方法的话，第二个参数要设置成"application/x-www-form-urlencoded"，否则post参数无法传递。

一种是使用http.PostForm方法

``` {.go}
func httpPostForm() {
    resp, err := http.PostForm("http://www.01happy.com/demo/accept.php",
        url.Values{"key": {"Value"}, "id": {"123"}})

    if err != nil {
        // handle error
    }

    defer resp.Body.Close()
    body, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        // handle error
    }

    fmt.Println(string(body))

}
```

复杂的请求，需要传递cookie,header等数据

``` {.go}
func httpDo() {
    client := &http.Client{}

    req, err := http.NewRequest("POST", "http://www.01happy.com/demo/accept.php", strings.NewReader("name=cjb"))
    if err != nil {
        // handle error
    }

    req.Header.Set("Content-Type", "application/x-www-form-urlencoded")
    req.Header.Set("Cookie", "name=anny")

    resp, err := client.Do(req)

    defer resp.Body.Close()

    body, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        // handle error
    }

    fmt.Println(string(body))
}
```

如果又要设置header又要设置post from的参数，可以按照以下格式写：

``` {.go}
func(){
    v := url.Values{}
    v.Set("testKey", "hello world")
    body := ioutil.NopCloser(strings.NewReader(v.Encode())) //把form数据编下码
    client := &http.Client{}
    req, _ := http.NewRequest("POST", "http://heiheihei", body)
    req.Header.Set("Content-Type", "application/x-www-form-urlencoded") //这个一定要加，不加form的值post不过去，被坑了两小时
    fmt.Printf("%+v\n", req)                                                         //看下发送的结构
    resp, err := client.Do(req) //发送
    defer resp.Body.Close()     //一定要关闭resp.Body
    data, _ := ioutil.ReadAll(resp.Body)
    fmt.Println(string(data), err)
}
```

如果像要设置host，参考golang中的文档：

| // If a server received a request with header lines,
|  //
|  // Host: example.com
|  // accept-encoding: gzip, deflate
|  // Accept-Language: en-us
|  // fOO: Bar
|  // foo: two
|  //
|  // then
|  //
|  // Header = map\[string\]\[\]string{
|  // \"Accept-Encoding\": {\"gzip, deflate\"},
|  // \"Accept-Language\": {\"en-us\"},
|  // \"Foo\": {\"Bar\", \"two\"},
|  // }
|  //
|  // For incoming requests, the Host header is promoted to the
|  // Request.Host field and removed from the Header map.

不能将host设置到req.Header中，而是使用req.Host来进行设置。

postman如果要设置host需要下载postman
interceptor插件，并在postman中启用。

query
=====

如果要传递url?xx=xx 类似这样的请求，需要使用以下方法:

``` {.go}
q := req.URL.Query()
q.Add("name", "xxx")
q.Add("para2", "xxx")
req.URL.RawQuery = q.Encode()
```
