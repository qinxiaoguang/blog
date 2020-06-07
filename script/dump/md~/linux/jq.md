一
==

一般在linux里格式化json的时候，都是使用命令 `python -m json.tool`
来进行的，如 `cat tmp | python -m json.tool`
，偶然间看到同事格式化很快捷，只键入的两个字母，就格式化了，而且还带颜色，于是学习了一下，这个工具就是jq,文档地址在这https://stedolan.github.io/jq/manual/,
使用方式也很简单，如 `cat tmp | jq .` ，如果想要json输入为一行，可以使用
`cat tmp | jq -c .` 注意后边有一个 `.` 号。
