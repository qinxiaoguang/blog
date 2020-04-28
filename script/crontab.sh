# 定时,每天凌晨备份一次
output=$1
cwd=$(cd `dirname $0`;pwd);
while true;do
    tomorrow=`date -d "+1day" +%Y-%m-%d`
    zerotime=`date -d "$tomorrow 00:00:00" +%s`
    now=`date +%s`
    interval=$(( zerotime-now ))
    sleep $interval
    cd $cwd && sh -x ./backup.sh $output
done
