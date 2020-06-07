背景
====

本机的一个项目是静态网页，使用 `file://`
的方式去访问，因部分插件的影响，会访问出错，所以需要通过host:ip的方式去访问，所以需要把静态网页进行部署，在本机上部署的方式可以使用tomcat,但是tomcat非常不方便，所以还是考虑使用nginx,顺便把使用方式记录下来。

install
=======

mac : `brew install nginx`

启动/停止
=========

-   `sudo nginx` :启动,之后浏览器访问 `http://localhost:8080` 即可。
-   `sudo nginx -s reload` :进行重启
-   `sudo nginx -s stop` :停止

配置
====

nginx配置文件位于 `/usr/local/etc/nginx/nginx.conf` 下。
详细配置:<http://www.nginx.cn/76.html>

worker_processes
-----------------

指明nginx开启的进程数，一般为CPU核数的1\~2倍，通过\`cat /proc/cpuinfo\|
grep \"cpu cores\"\| uniq\`查看CPU核数。

worker_cpuaffinity
-------------------

这个要结合 `worker_processes`
来使用，进程开了多少个，worker_cpuaffinity后的参数就有多少个，比如:

``` {.nginx}
worker_processes     2;
worker_cpu_affinity 01 10;
```

其中01表示启用第一个核，10表示启用第二个，依次类推，100就表示第三个，当然示例中不会出现这个。那么第一个进程就对应着01,第二进程对应10，也就是说第一个进程使用第一个内核，第二个进程使用第二个内核。

``` {.nginx}
worker_processes     2;
worker_cpu_affinity 0101 1010;
```

上述示例表示，第一个进程对应第一和第三个内核，第二个进程对应第二个和第四个内核。

worker_rlimitnofile
--------------------

worker_rlimitnofile
更改worker进程的最大打开文件数限制。如果没设置的话，这个值为操作系统的限制。设置后你的操作系统和Nginx可以处理比"ulimit
-a"更多的文件，所以把这个值设高，这样nginx就不会有"too many open
files"问题了。

error_log
----------

nginx的 `error_log` 类型如下（从左到右：debug最详细 crit最少）： \[
debug \| info \| notice \| warn \| error \| crit \]

``` {.nginx}
error_log logs/error.log notice
```

表示以notice类型来记录错误日志，打印到error.log中。
如果想要关闭日志功能，需要配置 `error_log /dev/null notice`

pid
---

``` {.nginx}
pid logs/nginx.pid
```

nginx.pid中放的是nginx开启的时候的进程号，所以如果更改进程号的时候，可以到这个地方更改。

Events 模块
-----------

``` {.nginx}
events {
  worker_connections 2048;
  accept_mutex off;
   use epoll
}
```

worker_connections
设置可由一个worker进程同时打开的最大连接数。总的最大链接数是进程数量和worker_connections的乘积。如果设置了上面提到的worker_rlimitnofile，我们可以将这个值设得很高。但是这个还有其他原因限制，所以不能不切实际的设置很高

accept_mutex如果打开了，连接会以串行方式运行，一个Worker进程被唤醒，其他进程保持休眠，如果没有打开，所有的worker被唤醒，不过只有一个Worker获取链接，其他的Worker重新进入休眠状态，这就是[惊群问题]。
但是如果连接数量少的时候，一般是建议打开accept_mutex的，但是如果连接数量比较多的时候，是建议关闭的。

use epoll,其有 `use epoll/poll/select/kqueue` 等，一般使用epoll

http模块
--------

nginx配置的核心，http模块控制着nginx resolver指定DNS服务器
sendfile表示高效传输文件，开启后可以让Nginx直接在磁盘和Tcp
Socket之间进行传输数据，如果在关闭状态，磁盘的数据首先会到用户空间的内存buf中，之后到内核空间的buf中，最后才到TcpSocket中。
如果是下载文件等内容，可以设为off.
参考:<http://www.linuxidc.com/Linux/2014-05/102321.htm>

``` {.nginx}
    # 开启sendfile，从磁盘读取文件后直接发送到网卡缓冲区，减少用户态和内核态的数据拷贝                                               
    sendfile        on;
    # 同时设置了两个值的话，将会在第一个buf发送的时候，强制push数据，而第二个buf时，将会调用tcp_cork来打开nagle算法，也就是后面的都会
应用tcp_nopush
    tcp_nopush     on;
    tcp_nodelay on;
    # 请求nginx时，http header如果想要支持下划线的话，需要增加如下配置
    underscores_in_headers on;
    # 该配置定义了正常情况下，nginx接收用户请求中http header(包括http头和http行)时分配的内存buffer大小，超过这个大小的时候，large_cli
ent_header_buffers配置将生效
    client_header_buffer_size 4k;
    # 该配置定义了nginx接收一个超大http header(包括http头和http行)，用到的buffer个数和buffer大小
    large_client_header_buffers 4 8k;
    # 该配置项定义了nginx接收http包体的最大限制
    client_max_body_size 21m;
    #该配置项定义了nginx接收http包体的内存缓冲区大小
    client_body_buffer_size 4m;
    # keepalive_timeout  0;
    # 一个keepalive连接在闲置一定时间后，会关闭这个连接，nginx默认是75秒
    keepalive_timeout  30;
    # 一个keepalive连接最多处理多少个请求，nginx默认是100 !lighttpd是16
    keepalive_requests 100;
    # 在接收客户端header过程中，如果超过一定时间没读取到客户端发过来的数据，则认为是超时，向客户端返回408，默认60s
    client_header_timeout 30; 
    # 在接收客户端body过程中，如果超过一定时间没读取到客户端发过来的数据，则认为是超时，默认60s
    client_body_timeout 300; 
    # 在向客户端发送数据的过程中，如果客户端超过一定的时间没有去接收这个数据包，那么nginx会关闭这个连接
    send_timeout 240;     
    # 对静态文件进行缓存，指定缓存的最大数目为1024个，如果缓存溢出，将会使用LRU进行淘汰，指定缓存文件被移出的时间1s
    open_file_cache max=1024 inactive=1s;
```

重要的配置在http的server节点下:

``` {.nginx}
#设定虚拟主机配置
    server {
        #侦听80端口
        listen    80;
        #定义使用 www.nginx.cn访问
        server_name  www.nginx.cn;

        #定义服务器的默认网站根目录位置
        root /home/web;

        #设定本虚拟主机的访问日志
        access_log  logs/nginx.access.log  main;

        #默认请求
        location / {

            #定义首页索引文件的名称
            index index.php index.html index.htm;   

        }

        # 定义错误提示页面
        error_page   500 502 503 504 /50x.html;
        location = /50x.html {
        }

        #静态文件，nginx自己处理
        location ~ ^/(images|javascript|js|css|flash|media|static)/ {

            #过期30天，静态文件不怎么更新，过期可以设大一点，
            #如果频繁更新，则可以设置得小一点。
            expires 30d;
        }

        #PHP 脚本请求全部转发到 FastCGI处理. 使用FastCGI默认配置.
        location ~ .php$ {
            fastcgi_pass 127.0.0.1:9000;
            fastcgi_index index.php;
            fastcgi_param  SCRIPT_FILENAME  $document_root$fastcgi_script_name;
            include fastcgi_params;
        }

        #禁止访问 .htxxx 文件
            location ~ /.ht {
            deny all;
        }

    }
```

防盗连
------

``` {.nginx}
location ~ .(jpe?g|png|gif)$ {
     valid_referers none blocked mysite.com *.mysite.com;
     if ($invalid_referer) {
        return   403;
    }
}
```

用 ("\|")
来分隔你想保护的文件的扩展名,valid~referers指令包含允许访问资源的网站列表~,不在列表中请求的返回403

除了使用location对文件访问进行限制，也可以对特定目录进行限制，下面的配置会禁止访问images目录下所有文件:

``` {.nginx}
location /images/ {
     valid_referers none blocked mysite.com *.mysite.com;
     if ($invalid_referer) {
        return   403;
    }
}
```

upstream
--------

upstream配置负载均衡

匹配规则
--------

-   \~ 波浪线表示执行一个正则匹配，区分大小写
-   \~\* 表示执行一个正则匹配，不区分大小写
-   \^\~
    表示普通字符匹配，如果该选项匹配，只匹配该选项，不匹配别的选项，一般用来匹配目录
-   = 进行普通字符精确匹配
-   @ \"@\" 定义一个命名的 location，使用在内部定向时，例如 error~page~,
    try~files~

location匹配优先级:

1.  =前缀的指令严格匹配这个查询。如果找到，停止搜索。
2.  所有剩下的常规字符串，最长的匹配。如果这个匹配使用\^〜前缀，搜索停止。
3.  正则表达式，在配置文件中定义的顺序。
4.  如果第3条规则产生匹配的话，结果被使用。否则，使用第2条规则的结果。

如：

``` {.nginx}
location  = / {
  # 只匹配"/".
  [ configuration A ] 
}
location  / {
  # 匹配任何请求，因为所有请求都是以"/"开始
  # 但是更长字符匹配或者正则表达式匹配会优先匹配
  [ configuration B ] 
}
location ^~ /images/ {
  # 匹配任何以 /images/ 开始的请求，并停止匹配 其它location
  [ configuration C ] 
}
location ~* .(gif|jpg|jpeg)$ {
  # 匹配以 gif, jpg, or jpeg结尾的请求. 
  # 但是所有 /images/ 目录的请求将由 [Configuration C]处理.   
  [ configuration D ] 
}
```

摘抄自:<http://www.nginx.cn/115.html>

常见问题
========

nginx路由转发
-------------

``` {.nginx}
location /test/ {
    proxy_pass http://example.com:protmail/;
}
```

# gzip压缩
网络中有时候会有很大的图片，需要对其进行一定的压缩，使用gzip即可进行压缩，配置如下:
```nginx
# 位于http模块下
gzip on; # 开启gzip压缩
gzip_min_length 1k; # 文件大小大于1k时压缩
gzip_buffers 4 16k; # 设置用于处理请求压缩的缓冲区数量和大小
#gzip_http_version 1.0; # 默认1.1，1.0不压缩，如果有需求，打开此注释
gzip_comp_level 2; # 压缩等级，数字越大压缩越好，耗时越长
gzip_types text/plain application/x-javascript text/css application/xml text/javascript application/x-httpd-php image/jpeg image/gif image/png; # 压缩的文件类型
gzip_vary off; # 
gzip_disable "MSIE [1-6]\."; # 浏览器禁用
```