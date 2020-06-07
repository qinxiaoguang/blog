查找规则
========

1.  优先查找xxx.rs 文件
    1.  main.rs、lib.rs、mod.rs中的mod xxx; 默认优先查找同级目录下的
        xxx.rs 文件；
    2.  其他文件yyy.rs中的mod xxx;默认优先查找同级目录的yyy目录下的
        xxx.rs 文件；
2.  如果 xxx.rs 不存在，则查找 xxx/mod.rs 文件，即 xxx 目录下的 mod.rs
    文件。

两条规则的加载效果是相同的。
