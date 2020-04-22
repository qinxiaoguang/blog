# 定时,每天凌晨备份一次
cwd=$(cd `dirname $0`;pwd);
while true;do
    tomorrow=`date -d "+1day" +%Y-%m-%d`
    now=`date +%s`
    interval=$(( tomorrow-now ))
    sleep $interval
    cd $cwd && nohup sh -x ./backup.sh &
done
