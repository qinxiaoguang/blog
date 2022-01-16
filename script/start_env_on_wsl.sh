#!/bin/bash
NGINX_BIN="nginx -c /home/qxg/proj/blog/script/nginx_local.conf"
MONGO_BIN="mongod --config /etc/mongodb.conf --fork"
REDIS_BIN="redis-server /etc/redis/redis.conf"
# nginx启动
cnt=`ps aux|grep "${NGINX_BIN}" | grep -v grep | wc -l`
if [ $cnt -eq 0 ];then
	sudo $NGINX_BIN
fi
# mongo启动
cnt=`ps aux|grep "${MONGO_BIN}" | grep -v grep | wc -l`
if [ $cnt -eq 0 ];then
	sudo $MONGO_BIN
fi


# redis启动
cnt=`ps aux|grep redis| grep -v grep | wc -l`
if [ $cnt -eq 0 ];then
	sudo $REDIS_BIN
fi
