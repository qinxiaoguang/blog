概述
====

docker是容器，k8s是管理容器的。

安装
====

Windows except 10: https://get.daocloud.io/toolbox/ Linux:apt install
docker.io Linux下安装完成后，还要运行daemon `service docker start` Max:
brew cask install docker
mac在使用前需要打开/Application/Docker.app/Mac/Docker
来登录，之后才能在命令行运行。 镜像：https://hub.docker.com/

使用
====

拉取镜像
--------

cmd: `docker pull [OPTIONS] NAME[:TAG]`
拉取镜像，name是拉取镜像名称，默认:lasted. eg: `docker pull hello-world`

查看本机镜像
------------

cmd: `docker images [OPTIONS] [REPOSITORY[:TAG]]` 查看本机有哪些镜像

运行镜像
--------

`docker run [OPTIONS] IMAGE[:TAG] [CMD] [arg ...]` 运行镜像 eg:
`docker run hello-world`

运行复杂镜像nginx:

1.  `docker pull nginx` : 拉取镜像
2.  `docker run -d nginx` : 后台运行容器
3.  `docker exec -it <id> bash` : 进入容器的内部，可以通过
    `docker exec --help` 查看该命令的说明

docker ps
---------

查看当前运行的docker容器,可以看到各种信息，如端口映射。

docker stop \<id\>
------------------

停止容器

docker run -d -p 8080:80 nginx
------------------------------

运行nginx，后台运行并且将主机的8080映射到容器的80端口。
默认运行容器使用的是Bridge模式，这个模式需要host主机和容器进行端口映射，即上述所述的。这样完成之后，就可以通过localhost:8080来访问镜像。

docker cp
---------

`docker cp localdir:dockerdir` 可以将本地的包文件发送到docker中。
而拷贝完后，再重启项目，原有的文件就会不再了。
但可以通过挂载的方式重启，即以下命令：
`docker run -d -v /localdir:/dockerdir -p 8080:80 ${docker_name}`

制作镜像
--------

第一步 `vim Dockerfile`:
为了描述方便，以下\#表示注释(实际操作并没有)，但是其实Dockerfile是支持这种注释的

``` {.docker}
FROM <BASE-IMAGE>   # 表示以某个镜像作为基础镜像，比如from tomcat，表示以tomcat这个镜像作为基础镜像
MAINTAINER qxg # 表示所有者，可不写
COPY <LOCAL FILE > /usr/local/tomcat/webapps # 将本地文件拷贝到容器的目录下
RUN apt update && apt install vim   #执行相应的命令，需要以RUN开头，
EXPOSE 80 # 该指令告诉容器，容器的应用程序会使用80端口，表示向外公开80端口
```

完了之后运行 `docker build .`
自动查找某目录的Dockerfile文件并构建，这样就构建成功了。 使用
`docker images`
查看当前镜像，会发现有一行镜像的名称和版本都是none，这个镜像就是刚build的。一般build的时候都会起上名字：
`docker build -t NAME:TAG`

看书笔记
--------

-   `docker run -i -t ubuntu /bin/bash`
    :执行ubuntu镜像，如果本地镜像不存在会到仓库中查找，如果仓库中存在自动下载，完成之后运行该景象，并在镜像中执行/bin/bash来启动一个shell。
-   `docker ps -a` 查看所有镜像，包括运行和停止的，而 `docker ps`
    只能查看运行镜像
-   `docker run --name myname -i -t ubuntu /bin/bahs`
    启动容器的时候，顺便给其命名，命名有助于区别不同的容器
-   `docker start [id]` 会启动一个容器，只是启动
-   `docker attach [id]` 附着对应的容器会话shell上
-   `docker run -d ubuntu /bin/sh -c "while true; do echo hello world; sleep 1; done"`
    会以守护进程的方式运行容器，长期在后台执行后边的命令，也就是会一直打印hello
    world
-   `docker logs [id]`
    可以查看对应的容器的logs，比如上一条中一直打印hello world的log
-   `docker -f logs [id]` 类似 `tail -f` 命令，持续监控log日志
-   `docker logs --tail 10 [id]` 查看最后10行日志
-   `docker logs --tail 0 -f [id]` 追踪最新日志
-   `docker top [id]` 查看容器的top
-   `docker status [id]` 查看容器的统计信息
-   `docker run [id/name] --restart`
    --restart会在容器异常停止的时候自动重启
-   `docker inspect [id]` 查看容器的更详细信息
-   `docker rm [id]` 删除容器
-   `docker rmi [name]` 删除镜像
-   `docker pull [name:tag]` 拉取镜像，其实就是下载镜像
-   `docker search [regx name]` 在hub仓库中查找镜像
-   `docker port [id] 80` 查看对应容器的端口号和外部映射的情况

-   `docker push [username/path]` 推送镜像到hub中

Dockfile的指令
--------------

-   `CMD` :指定容器运行的时候运行的执行,eg `CMD ["/bin/bahs","-l"]`
    ,但是docker run命令如果指定了运行时的指令，将会覆盖CMD指定的指令
-   `WORKDIR` 指定容器的工作目录，CMD指定的命令会在该目录进行执行，如
    `WORKDIR /opt/webapp`
-   `ENV` 设置容器的环境变量 `ENV MY_PATH /home/path`
    ,而设置多个环境变量的方式为:
    `ENV MY_PATH1=/home/path1 MYPATH2=/home/path2`
-   `USER`
    指定启动的时候用哪个用户去执行，这个用户是宿主机中的用户，如果不指定则会使用root用户去执行。\`USER
    qxg\`
-   `VOLUME` 给容器添加卷，可以理解为共享的磁盘，多个容器间可以共享。
    `VOLUME /opt/share`
-   `ADD`
    将本地环境的文件和目录复制到镜像中，如果是压缩文件，则会解压缩再复制过去
-   `COPY` 复制，但是不能解压缩
-   `SIGNSIGNAL` 停止容器的时候发送什么系统调用信号给容器
-   `FROM` 每个Dockfile都以FROM开头，以某个镜像为基础镜像

demo
----

1.  `mkdir decoker-demo` :创建项目名
2.  `cd docker-demo`
3.  `mkdir html`
4.  `echo '<h1>hello docker</h1>' > html/index.html`
5.  `echo -en 'FROM nginx\nCOPY html/* /usr/share/nginx/html\n' > Dockerfile`
6.  `docker build -t docker-demo:0.1 .` :构建
7.  `docker image ls` 查看刚创建好的镜像
8.  `docker run --name docker-demo -d -p 8080:80 docker-demo:0.1`
    :运行并将宿主的8080端口映射到容器的80端口。
9.  `docker container ps` 查看正在运行的容器
10. 浏览器输入 `http://localhost:8080` 查看
11. `docker container stop docker-demo` 停止容器
12. `docker container rm docker-demo` 删除容器
