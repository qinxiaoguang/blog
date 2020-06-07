背景
====

使用gitpage搭建了个人网站，并利用域名q.qxgzone.com进行重定向，但是国内访问的时候速度非常慢，于是采用国内重定向coding,国外重定向github的方式。

方法
====

1.  假设原仓库已经部署到github，现需要同时部署到coding上去，则
    `git remote add coding <coding-address>` ,可通过 `git remote -vv` 或
    `./git/config` 下查看添加后的效果。
2.  `git pull coding master --allow-unrelated-historie`
    将新的远程地址与本地同步。
3.  分别进行push `git push origin master` 、 `git push coding master`
4.  若想要一次性都进行push,则需
    `git remote set-url --add origin <coding-address>`
    来为origin添加新的远程仓库。可在./git/config下的origin下查看效果。之后运行
    `git push` 就同时push到两个远程仓库了。
