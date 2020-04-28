基础
====

go中的数据类型是静态的，如 `type Myint int`
Myint就是一个静态类型。他不是int，他就是Myint。
但go中一个类型的数据包含两个基础信息，一个是Type,一个是Value.
其中Type就是当前对象的静态类型是什么，而Value就是当前对象的值是什么。这个理解起来很简单，如:
\`var i interface{}; i=Myint(1)\`那么i的Type就是Myint,而Value就是1

相关函数
========

-   reflect.TypeOf() : 获取某个对象的Type(reflect.Type)
-   reflect.ValueOf() : 获取某个对象的Value(reflect.Value)

Value相关函数
-------------

Value通常包含以下几种数据:

-   基础类型，如int,float64等
-   函数类型, func(...)...
-   struct类型

相关函数：

-   Kind() :
    获取该value对应的类型名，被写死到了reflect包中，如reflect.Uint,
    reflect.Float64等。
-   NumField() : 获取操作对象的字段的个数, 如
    struct{int,string}就有两个字段
-   Field(i) : 获取操作的对象中第i个字段的底层Value
-   NumMethod()
    :与NumField不同的是，该方法获取的是操作对象的公开方法个数。
-   Elem() :
    如果操作的对象是一个指针，那么使用Elem()就可以将指针对应的底层值取出来。所以一个Value是指针的话，想要获取其值，就有必要使用Elem()来将底层Value取出来，那么如何判断Value是指针呢？使用Kind()方法
-   SetXX() :
    对操作对象修改其值，要求该操作对象是一个指针，否则修改不成功。
-   CanSet() : 判断操作对象能否修改底层的值。
-   Interface() : 将Value变为interface{},
    这样就可以通过类型断言来恢复其原有的样子，如
    `var i int; i = reflct.Value(i).Interface().(int)`
-   Call() : 如果Value是一个方法，则可以通过此方法进行调用。

相关用法
========

通过反射获取底层的某类型
------------------------

``` {.go}
type Test struct {
    NeedGet int  // 待获取数据
}
var t interface{}
t = Test{1}
// 如果底层数据是指针
reflect.ValueOf(&t).Elem().Field(0).Interface()
// 如果底层数据非指针
reflect.ValueOf(t).Field(0).Inteface()
```

通过反射调用其中的方法
----------------------

通过反射修改底层值
------------------

判断是否是某类型
----------------

不需要使用 等号来判断，而是使用类型断言直接判断，如:

``` {.go}
type Test struct {

}

var t interface{}
t = Test{}

tt, ok := t.(Test)
if ok {
    // 是Test类型
} else {
    // 非Test类型
}
```

也可以使用switch来判断,如

``` {.go}
switch t.(type) {
    case Test:
    dosomething()
}
```

关于unsafe
==========

1.  uintptr和unsafe.Pointer的区别

反射中的一些技巧
================
