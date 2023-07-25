output=$1 # 项目执行路径
cwd=$(cd `dirname $0`;pwd)
uploadimg=$cwd/../web/static/img/upload
cd $cwd
# 首先判断mongo中的数据有没有更新，有的话再进行更新
updatetime=`mongo --eval 'db.article.find({},{"last_publish_time":1}).sort({"update_time":-1}).limit(1)' blog | grep -oP "(?<=last_publish_time).*" | grep -oP "\d*"`
lasttime=`cat ./backtime | head -1`
if [ $updatetime -eq $lasttime ];then
	echo "no update"
	exit 0
fi

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
git pull && git add ./* && git add $uploadimg  && git commit -m "backup db"  && git push origin master

if [ $? -eq 0 ];then
	# 执行成功，将更新时间写入backtime中
	echo $updatetime > ./backtime
fi


