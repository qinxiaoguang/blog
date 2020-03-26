# redis backup
# redis-cli save
cwd=$(cd `dirname $0`;pwd)
cd $cwd
dump_file=$1
redis-cli bgsave

sleep 10s # 默认按照10s dump完毕

cp $dump_file ./dupm/redis/

