# sudo systemctl start redis
# https://zhuanlan.zhihu.com/p/34527270
# 使用编译方式安装，redis.conf在安装文件中，复制一份即可。
# start: cd ./dump/redis && redis-server /etc/redis.conf  这样就不用再cp dump.rdb文件了
# 其实最好的方式还是把部署流程及启动方式写到脚本里
