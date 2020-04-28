中文本换行符替换为\<br\>
========================

td中的数据如果有换行一般不会解析为换行，所以有的时候需要使用js将这些换行符换成对应的\<br\>标签

``` {.javascript org-language="js"}
var string = "哈哈哈\r\n"
string = string.replace(/\r\n/g,"<br>").replace(/\n/g,"<br>");
```

url encode/decode
=================

``` {.javascript org-language="js"}
var input = "嘿！哈！  "; 
encodeInput = encodeURIComponent();  // 编码
input = decodeURIComponent(encodeInput); //解码
```

标签操作
========

获取指定标签
------------

``` {.javascript org-language="js"}
inputs = document.querySelectorAll("input#su.bg.s_btn") //获取input标签且id为su,class为bg,s_btn的标签,注意返回的是数组
```

修改标签参数
------------

``` {.javascript org-language="js"}
inputs = document.querySelectorAll("input#su.bg.s_btn");
// 获取标签参数
onclickValue = inputs[0].getAttribute("onclick");
// 修改标签参数
inputs[0].setAttribute('value',"百度两下");
// 修改样式
inputs[0].style.backgroundColor = "red";
```

字符串包含另一字符串
====================

``` {.javascript org-language="js"}
first = "hhhhha";
sub = "hh";
if (first.indexOf(sub) != 1){
    console.log("包含")
}
```

获取当前网页的host/path等
=========================

``` {.javascript org-language="js"}
host = location.host; 
pathname = location.pathname; // 如www.baidu.com/s返回 /s,若无path则返回/
```
