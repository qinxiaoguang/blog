# blog
rust (acitx-web) 开发的个人博客

# 地址
https://qxgzone.com

# 使用方式
本项目依赖redis及mongodb以及nginx.
首先确保已安装redis及mongodb.
启动的配置位于conf/app.toml文件下.

## mongodb
mongodb内需要创建blog数据库以及article/media/quota,如果懒得折腾,到script/dump目录下,把个人的数据直接恢复到自己的mongo里即可,恢复方法参考:
```
mongorestore -h IP --port 端口 -u 用户名 -p 密码 -d 数据库 文件存在路径
```
其中redis也一样.
## nginx
将80端口映射到本项目的web目录下,确保http://localhost 能够访问到web目录下的网页.
个人的配置在这里,写得很烂,可以参考下: https://www.qxgzone.com/article/detail/5ea147c2007c2f4d00d70b56
本地测试可以直接使用`script/nginx_local.conf`文件，其中前端使用80端口，后端使用8080端口。
如果需要更改localhost为ip的方式测试，则需要修改`conf/app.toml`及`web/static/js/base_dev.js`两个文件。

如果需要开启wasm，需要在配置文件中的mime.types加上`application/wasm wasm`
## github第三方登录
因为本项目是自用的,对文章进行添加及编辑是需要采用第三方登录,其中服务器会硬编码校验登录的人是不是我本人,若是则会开放编辑权限.
如果需要编辑,则在代码中,将对应的权限给放开即可,代码位于blog/src/common/user_helper.rs下,将登录用户修改为本人即可.

登录页面很简陋,为: http://localhost/login.html


## 运行
redis/mongodb/nginx都准备就续后,调用脚本`sh run.sh dev` 来启动服务器即可.
