# run.sh ${env}
# env can be: dev/test

cwd=$(cd `dirname $0`;pwd)
jsdir=$cwd/web/static/js
projname="blog"
debugfile=$cwd/target/debug/$projname
releasefile=$cwd/target/release/$projname
output=$cwd/output

rm -rf $output
mkdir -p $output

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
    cp -r $cwd/conf_online $output/

    cp $jsdir/base_online.js $jsdir/base.js

    # 执行 备份
    cd $cwd && nohup sh ./crontab.sh >> /dev/null &

    # run 
    cd $output && ./$projname
}

case $1 in
    dev)
	dev
    	;;
    online)
	online
	;;
    *)
	dev
	;;
esac

