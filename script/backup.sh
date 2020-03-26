cwd=$(cd `dirname $0`;pwd)
cd $cwd
proj_path=$1 # 本地的proj路径
# 第二个参数是mongo的ip,第三个参数是mongo的port

sh ./mongo_backup.sh $2 $3
sh ./redis_backup.sh

# 将dump目录mv到本地的github项目路径，并执行git add ./ git commit / git push命令
mv ./dump $proj_path
cd $proj_path 

# 在执行，一定要配置ssh-gen,否则执行失败
git add .  && git commit -m "backup db"  && git push origin master

