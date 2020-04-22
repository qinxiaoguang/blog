# redis backup
# redis-cli save
cwd=$(cd `dirname $0`;pwd)
cd $cwd
redis-cli bgsave

sleep 10s # 默认按照10s dump完毕

dir=`redis-cli CONFIG GET dir | grep -v dir`
cp $dir/dump.rdb $cwd/dump/redis/

# 恢复的时候，先停掉redis,再复制dump.rdb文件，再重启redis
