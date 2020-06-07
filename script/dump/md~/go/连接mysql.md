概述
====

使用 `github.com/go-sql-driver/mysql` 进行连接。 另一种方法是使用 gorm:
`https://github.com/jinzhu/gorm` ,先不记录。

使用方法
========

初始化
------

``` {.go}
// 必须引入的两个库是 database/sql和 github.com/go-sql-driver/mysql
import (
    "database/sql"
    "fmt"
    "os"
    _ "github.com/go-sql-driver/mysql"
)

func initDB() {
    // USER 连接的用户名
    // PASSWORD 密码
    // HOST host
    // PORT port
    // DB_NAME 数据库名
    path := fmt.Sprintf("%v:%v@tcp(%v:%v)/%v?charset=utf8",
        USER, PASSWORD, HOST, PORT, DB_NAME)
    // 使用 mysql驱动，需要引入github.com/go-sql-driver/mysql初始化
    DB, _ = sql.Open("mysql", path)
    // 设置最大连接数
    DB.SetConnMaxLifetime(100)
    // 设置空闲连接数
    DB.SetMaxIdleConns(10)
    if err := DB.Ping(); err != nil {
        fmt.Printf("open database failed, err is %v\n", err)
        os.Exit(-1)
    }
    fmt.Println("open database success")
}
```

select
------

查找一个数据

``` {.go}
var jobGroup JobGroup
err = DB.QueryRow("select name,job_type from job_groups where id = ?", id).Scan(&jobGroup.Name, &jobGroup.JobType)
if err != nil {
    fmt.Printf("rows run failed : %v\n", err)
    continue
}
```

查找多个数据

``` {.go}
rows, err := DB.Query("select group_id, package_id, service_address from jobs")
if err != nil {
    fmt.Printf("query failed:%v\n", err)
}
for rows.Next() {
    var job Job
    err := rows.Scan(&job.GroupId, &job.PackageId, &job.ServiceAddress)
    if err != nil {
        fmt.Printf("rows run failed : %v\n", err)
        continue
    }
    // ...
}
```

...
---

lazy handle模式，用不上的先不记录，用上的时候再进行记录。
