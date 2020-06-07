概述
====

golang中可以通过json.Unmarshal将一个json字符串解为map对象，也可以解为struct对象，但json的嵌套足够多的时候struct同样嵌套很多，同时要创建很多struct结构，而有时候只是想获取json结构中的某一个值，写很多struct太复杂，所以考虑使用一个struct来获取其值

方式
====

使用匿名struct的方式，如

``` {.go}
type Person struct{
    Family struct {
        Dad struct {
            Name string `json:"name"`
            age int `json:"age"`
        } `json:"dad"`
    } `json:"family"`
}

type ArrayPerson struct {
    Persons []struct{
        Name string `json:"name"`
        Age int `json:"age"`
    } `json:"persons"`
}

var jsonValue = `
{ 
    "family":{
        "dad": {
            "name":"dad",
            "age":18
        }
    }
}
`

var arraryPersons = `
{
    "persons":[
        {
            "name":"haha",
            "age":18
        },
        {
            "name":"heihei",
            "age":19
        }
    ]
}
`

func main() {
    var  p Person
    var ps ArrayPerson
    json.Unmarshal([]byte(arraryPersons), &ps)
    json.Unmarshal([]byte(jsonValue), &p)
    fmt.Println(p)
    fmt.Println(p.Family.Dad)
    fmt.Println(ps)
}
```
