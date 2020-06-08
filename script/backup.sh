output=$1 # 项目执行路径
cwd=$(cd `dirname $0`;pwd)
uploadimg=$cwd/../web/static/img/upload
cd $cwd
# 第二个参数是mongo的ip,第三个参数是mongo的port

sh ./mongo_backup.sh 127.0.0.1 27017
sh ./redis_backup.sh

# markdown dump

cd $cwd
curl "localhost:8089/article/dump"
rm -rf ./dump/md~
# 将output的md进行压缩
cd $output && tar -zcvf md.tgz ./md
cd $cwd
mv -b $output/md.tgz ./dump/
rm -rf ./dump/md.tgz~

# 在执行，一定要配置ssh-gen,否则执行失败
git pull && git add . && git add $uploadimg  && git commit -m "backup db"  && git push origin master


