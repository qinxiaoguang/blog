cwd=$(cd `dirname $0`;pwd)
cd $cwd
# 第二个参数是mongo的ip,第三个参数是mongo的port

sh ./mongo_backup.sh 127.0.0.1 27017
sh ./redis_backup.sh

cd $cwd
# 在执行，一定要配置ssh-gen,否则执行失败
git pull && git add .  && git commit -m "backup db"  && git push origin master


