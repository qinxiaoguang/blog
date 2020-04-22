cwd=$(cd `dirname $0`;pwd)
echo $cwd
cd $cwd

ip=$1
port=$2
mongodump -h $1 --port $2 -d blog -o ./dump/mongo

# restore
# mongorestore -h IP --port 端口 -u 用户名 -p 密码 -d 数据库 文件存在路径
# mongorestore -h IP --port 端口 -u 用户名 -p 密码 -d blog ./dump/mongo/blog
# 在restore之前最好将collection的数据清空一下，否则id相同的无法restore
