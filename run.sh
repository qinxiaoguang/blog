# run.sh ${env}
# env can be: dev/test

cwd=$(cd `dirname $0`;pwd)
webdir=$cwd/web
jsdir=$webdir/static/js
projname="blog"
debugfile=$cwd/target/debug/$projname
releasefile=$cwd/target/release/$projname
output=$cwd/output

rm -rf $output
mkdir -p $output/tmpfile # tmpfile用于生成临时文件

function dev(){
    echo "dev"
    # first cargo build
    cargo build
    # cargo build --release
    if [ $? -ne 0 ];then
	echo "build failed"
	exit 0
    fi
    cp $debugfile $output
    cp -r $cwd/conf $output/

    # 修改js文件
    cp $jsdir/base_dev.js $jsdir/base.js
    # 修改登录文件
    cp $webdir/login_dev.html $webdir/login.html

    # run 
    cd $output && ./$projname
}

function online(){
    echo "online"
    # first cargo build
    cargo build --release
    if [ $? -ne 0 ];then
	echo "build failed"
	exit 0
    fi

    cp $releasefile $output
    cp -r $cwd/conf_online $output/conf

    cp $jsdir/base_online.js $jsdir/base.js
    cp $webdir/login_online.html $webdir/login.html

    # 执行 备份
    cd $cwd/script && nohup sh -x ./crontab.sh $output >> $output/crontab.log &

    # run 
    cd $output && ./$projname
}


function ip(){
    echo "ip"
    # first cargo build
    cargo build --release
    if [ $? -ne 0 ];then
	echo "build failed"
	exit 0
    fi

    cp $releasefile $output
    cp -r $cwd/conf_ip $output/conf

    cp $jsdir/base_ip.js $jsdir/base.js
    cp $webdir/login_ip.html $webdir/login.html

    # 执行 备份
    cd $cwd/script && nohup sh -x ./crontab.sh $output >> $output/crontab.log &

    # run 
    cd $output && ./$projname
}

function stop(){
    # 下掉crontab
    echo "stop"
    ps aux | grep "crontab.sh" | grep -v grep | awk '{print $2}' | xargs kill -9
    exit 0
}


trap 'stop' INT
case $1 in
    dev)
	dev
    ;;
    online)
	online
	;;
    ip)
    ip
	;;
    *)
	dev
	;;
esac

