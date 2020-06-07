区别
====

1.  const类似c中的\#define,会将const对应的数值硬编码到code中，而static则是变量，会在运行的时候进行初始化。
2.  const的值不允许被改变，而static的值可以被标记为mut,通过unsafe进行改变。但不建议对static的值进行更改。
