# centos 7
su # 需要root环境
cd -
wget http://download.redis.io/releases/redis-4.0.1.tar.gz
tar -zxvf redis-4.0.1.tar.gz
cd redis-4.0
make
make install

mv redis.conf /etc/ # redis.conf可以放在本地保存
