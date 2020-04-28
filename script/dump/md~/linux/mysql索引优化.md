概述
====

在sql执行方面,如果发现sql性能下降,其可能原因有执行时间长,或等待时间长.再细分就是:查询语句写的烂,索引失效,关联查询太多join,服务器调优及各参数设置

join
====

首先来说join,join一般分7种如下:
![](http://q.qxgzone.com/static/img/mysql索引优化_mysqljoin.png)
如何去记忆,首先对于两表关联,采用的方式为笛卡尔积,如TableA有4条记录,TableB有5条记录,那么最终的结果是20条记录,接着再套上述的七种查表方式,比如inner
join,内连接,其找的就是AB共有的部分,即20条记录中A.key =
B.key的部分,而对于left join而言,不但要找A.key =
B.key的部分,还要找A独有的部分,独有部分没有B部分,此时补空.那么接着是A独有,前边说过,A独有的部分,没有B部分,那么B部分的key一定为null,所以条件为left
join + where B.key = null. 在mysql中并没有full outer
join,但是通过union操作,可以将left join + right
join组成的数据进行合并,这样就组成了full outer
join.同样,最后一种,可以看图,是那两种的组合.

接着来说说,面试常问套路:什么是索引?

索引
====

官方定义:索引是帮助mysql高效获取数据的数据结构.可以看到,索引是一种数据结构.类比数组,就是下标.最精简的解释方式,就是:排好序的快速查找数据结构,这句话有两点知识,一是排序,二是快速查找.

在数据之外,数据库系统还维护着满足特定查找算法的数据结构,这些数据结构以某种方式指向数据.而对于索引,其键值中的值是一种指针,而不是对应数据.

在mysql中实现的索引一般都是Btree索引.

在使用索引的这种数据结构中,对于删除某条数据,一般不会真真正正的删除,而是将其中的激活位给消除,为什么不去删除,目的有两个,一个是为了数据分析,一个是为了不影响索引.

索引本身也很大,不可能全部存储在内存中,因此索引往往以索引文件的形式存储在磁盘上.

虽然索引提高查询速度,但是降低表的更新速度,每次调整数据,都要调整索引中的字段.

索引建立不是一朝一夕的,因为业务可能会随时改变.

索引分类
========

-   单值索引:一个索引只包含单个列,一个表可以有多个单列索引,一般不建议超过5个
-   唯一索引:索引列的值必须唯一,但允许有空值
-   复合索引:一个索引包含多个列,注意并不是三个索引同时进行排序,而是首先查索引第一个字段,再查询第二个,依次类推.

建立索引操作形式为:create \[unique\] index idx~username~ on user(name).

什么情况下需要建索引
====================

虽然索引优化查询速度,但是并不是说一定要建索引,是药三分毒,但是建索引也是有一些规则:
主键自动建索引,频繁查询字段建索引,查询中与其他表关联的字段建索引,排序字段建索引.

而对于频繁更新的字段不适合建索引,where条件始终用不到字段不创建索引,表记录太少不建,经常增删改的表不建,重复率高的字段不建,比如性别

一般创建索引会采用复合索引的模式.

性能分析
========

-   Mysql Query Optimizer:Mysql自带的查询优化器
-   Mysql常见瓶颈：CPU，IO，服务器硬件性能瓶颈（top,free,iostat,vmstat来查看系统的性能状态）
-   Explain:使用Explain关键字可以模拟优化器执行SQL查询语句，从而知道MySql是如何处理你的SQL语句，分析你的查询语句或是表结构的性能瓶颈。

Explain
=======

使用Explain关键字可以模拟优化器执行SQL查询语句，从而知道MySql是如何处理你的SQL语句，分析你的查询语句或是表结构的性能瓶颈,
用法是：explain + SQL语句。 eg: `explain select from user;` 打印信息：

``` {.shell}
mysql> explain select from user;
+—-+————-+——-+————+——+—————+——+———+——+——+———-+——-+
| id | select_type | table | partitions | type | possible_keys | key | key_len | ref | rows | filtered | Extra |
+—-+————-+——-+————+——+—————+——+———+——+——+———-+——-+
| 1 | SIMPLE | user | NULL | ALL | NULL | NULL | NULL | NULL | 2 | 100.00 | NULL |
+—-+————-+——-+————+——+—————+——+———+——+——+———-+——-+
1 row in set, 1 warning (0.00 sec)
```

其得到的信息如上，其表头的信息是什么？

-   id是select查询的序列号，包含一组数字，表示查询中执行select子句或操作表的顺序。值有三种（id相同：执行顺序由table上到下。id不同：id大的优先级越高，优先被执行。id有相同，也有不同：前两种规则的组合）
-   select~type~
    表示查询的类别，有SIMPLE(简单查询，不包含Union等查询)，PRIMARY（最后加载的查询），SUBQUERY（子查询，在括号的查询），DERIVED（可以理解为临时表），UNION，UNION
    RESULT等。
-   table 表示是哪个表
-   type
    有All(全表扫描),index(全表扫描，但是是全索引扫描),range(只检索给定索引的行，一般是在where中出现between,\>,\<，in等),ref(非唯一索引扫描，可能找到多个符合条件的行),eq~ref~(唯一索引扫描，只有一条记录匹配常见于主键或唯一索引中，常见于where
    id =
    ''这种形式中),\[const（表示通过索引一次就找到）,system(表中只有一行记录，几乎不出现)\],NULL.从最好到最差依次为system\>const\>eq~ref~\>ref\>range\>index\>All,优化时最好达到reange,或ref级别，百万级别出现All的时候，一般都需要进行优化
-   possible~keys和key~：是否使用到了索引，多个索引竞争时，用到了哪个索引。possible~keys表示可能用到的索引~，但不一定被实际使用。key表示使用的索引。通过这两种键，可以查看索引是否失效等。这里有个特例，如果使用select
    from
    user，那么其type就是All，possible~keys是NULL~，key是NULL，表示全表扫描，效率最低，但是如果采用select
    col1,col2 from
    user的形式，其possible~keys是NULL~，但是如果存在一个复合索引是col1,col2，那么此时key就将会是该索引，而type是index,这就解释了，为什么一般不建议使用select
    的形式。
-   key~len~：表示索引中使用的字节数，可通过该列计算查询中使用的索引的长度，在不损失精度的情况下，长度越短越好，其长度表示最大可能长度，不是实际长度。
-   ref: 显示索引的哪一列被使用了，通常是个常数 const.
-   rows:
    大致估算出，找出所需记录数需要扫描的行数，并不是精确数，即表示其值越小越好。
-   Extra:包含不适合在其他列显示的十分重要的信息,如Using
    filesort(无法利用索引完成的排序,即使用索引查询数据,但是order
    by时却没有使用索引,称为文件排序,比较坑爹,如果可以,尽快优化),Using
    temporary(使用了临时表保存了中间结果,在对查询结果进行排序时使用临时表,常见于排序order
    by和分组查询group by),Using
    index(表明用到了覆盖索引,如果所查字段和索引匹配,则使用了覆盖索引,性能较高,但是如果是select
    ,将不会出现using index,所以一般不建议使用select ),Using
    where(使用到了where过滤),imporssible
    where(表示where一直返回false),...重要的是前三个id，type,key,rows,Extra是最重要的字段属性。

在优化的时候,首先要解决type
All的情况,可以采用建立索引来优化,而对于key为Null,可以通过建立索引和修改索引以及修改查询条件中字段的顺序等来优化,对于Extra,如果出现Using
filesort,Using temporary必须要进行对order by等的优化.
对于左右连接等的优化,一般索引建立在相反的字段上,比如left
join的条件,一般会在右表建立索引.

索引失效
========

索引失效,并不是出现BUG,而是建立了索引,因为编写SQL语句问题,导致建的索引用不上.
常见的索引失效有:

-   全值匹配:假如建立索引的字段为A,B,C,同样其有顺序,那么对于使用的时候,where
    A='' and B='' and C='',这种情况,肯定会用到索引,而where A= ''
    ,也会用到索引,但是对于where
    B='',这种情况,是用不到索引的,可以这么理解,对于索引其顺序而言,必须匹配其前缀,如果部分匹配,则前边的部分有效,后边所有的索引均会失效.
-   最佳左前缀:其实就是对上述的总结,带头兄弟不能死,中间兄弟不能断.但是如果索引是ABC,而用的时候,使用的是CBA,这样也是会匹配的,原因是mysql的优化器会自动优化.
-   不在索引列上做任何操作(计算,函数,类型转换等),否则会导致索引失效而转向全表扫描.比如,不能将where
    name = 'lisi' 改为where left(name,4) = 'lisi',这样会导致索引失效.
-   范围之后全失效,如果某个字段使用了between,in,\>,\<,like等范围操作,那么此字段之后的(不包含此字段)索引都会失效.比如where
    A = '' and B\>'' and
    c='',那么对于ABC索引,其只有A索引有效,BC索引均失效.
-   尽量少用\*,而使用覆盖索引(只访问索引的查询),比如建立的索引是ABC,那么尽量使用select
    A,B,C去访问数据.
-   在使用!=或者\<\>时,无法使用索引会导致全表扫描.
-   is null,is not null无法使用索引
-   like以通配符开头('%')索引会失效.比如like('%lisi')索引失效,但是like('lisi%')索引并不会失效.那么此时出现问题,like
    '%lisi%'我就是想要,那么如何解决其索引失效的问题?解决办法就是使用覆盖索引,对于select后面跟的内容,如果被包含在覆盖索引中,那么就会使用到索引,比如有索引ABC,那么如果select
    A,B,C from table where A like '%lisi%'这种情况会使用到索引.
-   字符串不加单引号导致索引失效,原因内部做了隐含的类型转换，再看第三条就知道了。
-   少用or,用or的时候也会导致索引失效.

优化
====

对于优化的思路,一般不会直接使用explain进行优化,而是首先观察SQL情况,比如开启一天,然后开启慢查询日志,设置阙值,抓取慢SQL,接着才是使用explain进行优化,接着是show
profile,最后进行服务器参数调优. 所以对于整体的优化思路,
可以总结为以下四步:

1.  慢查询开启并捕获
2.  explain + 慢sql分析
3.  show profile查询SQL在服务器里面的执行细节和生命周期情况
4.  SQL数据库服务器参数调优.

首先慢查询开启,就是开启慢查询日志,默认10s以上的查询是慢查询.默认情况下,没有开启慢查询,如果不是调优的时候,不需要启动.可以通过
`show variables like '%slow_query_log%'` 来查看当前mysql慢查询状态:

``` {.shell}
+———————+————————————–+
| Variable_name | Value |
+———————+————————————–+
| slow_query_log | OFF |
| slow_query_log_file | /var/lib/mysql/qxg-GE60-2PL-slow.log |
+———————+————————————–+
```

那么开启方法为:set global
slow~querylog~`1,不过这种方式只对当前数据库生效,如果mysql重启后就失效了.
查看阙值: =show variables like 'long_query_time%'` :

``` {.shell}
mysql> show variables like 'long_query_time%';
+—————–+———–+
| Variable_name | Value |
+—————–+———–+
| long_query_time | 10.000000 |
+—————–+———–+
```

可以看到默认的阙值是10s,而设置阙值的方式是:set global
long~querytime~=3,这样就可以设置其阙值为3s.但是修改完后,再查询变量其还是10,但实际已经改变.
可以通过select sleep(4)来进行测试.

如果在生产的时候,慢查询日志中会有一堆的查询日志,那么最有利的分析工具是mysqldumpslow,该命令可以对数据进行分析,比如10分钟最多访问频率的数据等等.

批量数据脚本
============

为什么要使用批量数据脚本，对于数据分析而言，数据少的时候，一条sql语句的效率并不会看出其效率高低之分，反倒数据多的时候，查询的所消耗的时间才会更加明显，所以在分析过程，大量的数据一般是必须的。
首先是建表.
为了避免创建函数失败,需要指定log~bintrustfunctioncreators为1~,set global
log~bintrustfunctioncreators~=1;
然后创建函数,保证每条数据不同,那么这就需要mysql有随即字符串的函数,但是mysql并没有提供,所以需要我们自己去编写一个函数

``` {.mysql}
delimiter $$
create function rand_string(n int) returns varchar(255)
begin
    declare chars_str varchar(100) default 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
    declare return_str varchar(255) default '';
    declare i int default 0;
    while i<n do
        set return_str=concat(return_str,substring(chars_str,floor(1+rand()*52),1));
        set i=i+1;
    end while;
    return return_str;
end $$
```

接着创建存储过程:

``` {.mysql}
delimiter $$
create procedure insert_user(in max_num int(10))
begin
    declare i int default 0;
    set autocommit =0;
    repeat
        set i=i+1;
        insert into user(username,password,phone,car_id) values(rand_string(6),rand_string(4),rand_string(5),rand_string(1));
        until i=max_num
    end repeat;
    commit;
end $$
```

接着准备调用存储过程:

``` {.mysql}
delimiter ;
call insert_user(100);
```

其执行结果为下(包括之前有的数据):

``` {.shell}
+—–+———-+———-+————+——–+
| id | username | password | phone | car_id |
+—–+———-+———-+————+——–+
| 1 | qxg | a123456 | 1786278780 | kjalk |
| 2 | q123456 | a123456 | 12341 | 4124 |
| 3 | 啊啊 | aaa | aaa | aaa |
| 4 | ngUdIg | FKIj | VdCBc | G |
| 5 | XWQnUI | HmoH | ucdjM | g |
| 6 | uHZdsE | SChd | TlxFJ | D |
| 7 | OjGBOq | ntdm | arHiV | e |
| 8 | KlbAvA | smic | ONzgi | w |
| 9 | IclWcx | dZOV | nCWdB | w |
| 10 | CxGOaK | ylLY | IvekK | W |
| 11 | EgQNpk | ijvB | uvUmA | Q |
| 12 | DvsCIJ | tUqT | yMmar | H |
| 13 | jYsQbL | BzTR | GDYiS | S |
| 14 | JTqWMV | pOZG | gPFEi | d |
| 15 | PQHQfg | pgKi | JWEjg | f |
| 16 | fmUNgq | jZtW | BQBjp | Y |
| 17 | YVIDSF | wtBB | fUihk | D |
| 18 | oJBHHn | tfvM | zmMZM | L |
| 19 | vTijxK | gzgg | nUMZL | F |
| 20 | UHEXah | NtEQ | pcqzz | Z |
| 21 | xqkfVn | ANqq | HmpNT | g |
| 22 | XwodCA | Xkjn | OehwN | B |
| 23 | pWLTkr | gFHt | YNUhc | Q |
| 24 | XoDZot | bdnf | MvPPD | w |
| 25 | wVoIwk | MekL | bYQgh | u |
| 26 | BwEJFZ | jTTM | fnbrF | Z |
| 27 | hOxTdJ | nmun | fNxZD | U |
| 28 | NhvHXP | kErX | LLAqg | F |
| 29 | KKwaLF | TEpK | GZduN | H |
| 30 | WKMDHC | NgvH | afziq | g |
| 31 | IYQlEr | VBUW | WSyRL | f |
| 32 | uIgEFm | wtFV | NegtA | u |
| 33 | zmNegu | EMWz | FEfNB | q |
| 34 | ZZZbfz | hkDo | HtWFi | c |
| 35 | LAslbC | ErXK | GcsHj | Y |
| 36 | pFhVhZ | BILE | Qrkay | o |
| 37 | YbnklB | Xkhh | oXXUG | w |
| 38 | ncyitx | gnXW | RvApZ | b |
| 39 | hJWJCM | bVAQ | DtjPw | P |
| 40 | Kdmbxg | mTHG | hRPzd | R |
| 41 | YsSlDi | hiwG | RpbkV | X |
| 42 | cuRZyr | osXK | JoqOY | y |
| 43 | wMwVnE | jfZI | pzFzK | c |
| 44 | dneHZY | WNZI | rLFQp | Z |
| 45 | gGObNO | CxFG | tYNTg | X |
| 46 | vlTIKA | wHXO | cVyCr | b |
| 47 | hFFlqT | wCvw | VnEhY | t |
| 48 | aWGqKE | PoZe | wXvmX | d |
| 49 | waLFTB | cJle | NBsiP | y |
| 50 | Xtbbdk | QzZx | phSPu | H |
| 51 | cnicKv | ZHpC | QzXrP | Y |
| 52 | zBkvvU | jnNc | VwvND | E |
| 53 | jinQrk | ayqi | SNnbr | H |
| 54 | jXmtfw | SXmp | NTijv | B |
| 55 | wCvxZE | XZey | fgrow | n |
| 56 | annCZn | saZV | EmtjP | y |
| 57 | WnALfu | GYZa | fziso | p |
| 58 | GoyBnI | EWVO | gpfHT | w |
| 59 | BqduLv | XBNm | WXXVJ | K |
| 60 | xgqkdK | rFYa | jRKWF | k |
| 61 | kurAEu | kPvI | ewWte | o |
| 62 | iZvejJ | Rfdb | WCWbu | R |
| 63 | bGZhLj | NlTI | Jwgqk | f |
| 64 | TcGYXU | EoGp | EdCDj | j |
| 65 | tpudgx | SWdD | ICPsu | V |
| 66 | rYTxHS | tmgV | jioUG | v |
| 67 | mWWUJM | FSwG | RmNeg | s |
| 68 | vbSkwD | ATTJ | RhjzT | X |
| 69 | hROuHc | oloL | NJeuM | A |
| 70 | pYZbhF | HwkK | VwvQQ | I |
| 71 | TtngUc | DLUr | aaZZV | H |
| 72 | wofHUC | eOGR | nOiyT | W |
| 73 | eEKKwc | ZMOH | SusHg | K |
| 74 | iIRkBZ | uYKF | TDlwy | c |
| 75 | QWloPe | dcbb | bfuKp | v |
| 76 | hzbIor | SnMX | BNlPq | l |
| 77 | gYBIOV | jjso | qNQQI | S |
| 78 | nOegsv | YIua | RioVN | c |
| 79 | VyEzMn | aqAF | zJWHt | W |
| 80 | DdEOfl | Ngrr | HkfXv | k |
| 81 | Orotch | Duom | sciKY | P |
| 82 | bORSMf | rsPW | koNXB | N |
| 83 | kKXFjd | PROr | uYIua | Q |
| 84 | eZIuVx | xYxr | pCPtz | o |
| 85 | YZdqwk | KUtk | SJRjs | k |
| 86 | atUpSt | prQe | XAKaV | C |
| 87 | bztuSe | RVgQ | MojaC | L |
| 88 | XDarJt | RdPP | GJBHF | g |
| 89 | PGLNEH | AExx | WpJzx | N |
| 90 | vRVeJj | RGFg | TWfLm | c |
| 91 | DIHiXm | schF | Dczrl | e |
| 92 | OFKKwd | aQea | OUeOF | M |
| 93 | QWibIm | jkzR | MkLce | n |
| 94 | dEJIot | djKa | XJESD | m |
| 95 | BUVTFv | mZkb | DMaOW | n |
| 96 | CWbtMD | GvlS | GBNlR | C |
| 97 | jnMVsa | chFD | cAtuQ | W |
| 98 | kmCbAz | WhXq | MOIab | g |
| 99 | zhjAbF | VPjE | qTtpt | Z |
| 100 | RlEpJz | yRPw | RRISn | O |
| 101 | cYIxnV | OjDn | Hvfrt | R |
| 102 | cKvWyz | eXAI | RhnSx | M |
| 103 | rwhySW | dCBc | FTFsY | N |
| 104 | VpKHfE | IBJR | eVpNU | i |
| 105 | irjWfL | qwkK | XFiaB | H |
| 106 | HpCRDq | Stpu | baWHx | o |
| 107 | cxecYN | RUZo | wofKj | Q |
| 108 | zdRazy | SSNk | KVAOu | G |
| 109 | VKNLps | UsfA | kzRMh | A |
| 110 | gecbZR | nOdb | VyGMP | O |
| 111 | xZDSCl | vvSd | LwZHo | x |
| 112 | uIfAoS | wGSr | gFKHf | G |
| 113 | PefkLb | VBUV | TFvlW | W |
+—–+———-+———-+————+——–+
```

完了之后,就可以向里面插入100W条数据,并进行测试.

那么在存在了数据之后,就可以进行show profile进行调优. show
profile是mysql提供可以用来分析当前会话中语句执行的资源消耗情况(mysql分四层,连接,服务,引擎,存储,那么其每层消耗多久可以测量),可以用于sql的调优的测量.默认情况下,参数处于关闭状态,并默认保存最近15次的运行结果。

首先要查看当前mysql版本是否支持： `show variables like 'profiling'`
,可查看其默认是关闭状态。接着打开： `set profiling =on`
;然后开始运行sql,比如select from user;然后开始查看： `show profiles` :

``` {.shell}
mysql> show profiles;
+———-+————+———————————+
| Query_ID | Duration | Query |
+———-+————+———————————+
| 1 | 0.00113950 | show variables like ‘profiling’ |
| 2 | 0.25660775 | select from user |
+———-+————+———————————+
```

可以看到，每条的查询都会存储，并保存其时间。那么如果想要获得某条查询具体的信息，可以通过以下命令show
profile cpu,block io for query +id,（或show all for query
+id）注意id就是上述表格中的id值：

``` {.mysql}
mysql> show profile cpu,block io for query 2;
+———————-+———-+———-+————+————–+—————+
| Status | Duration | CPU_user | CPU_system | Block_ops_in | Block_ops_out |
+———————-+———-+———-+————+————–+—————+
| starting | 0.000044 | 0.000000 | 0.000000 | 0 | 0 |
| checking permissions | 0.000006 | 0.000000 | 0.000000 | 0 | 0 |
| Opening tables | 0.000011 | 0.000000 | 0.000000 | 0 | 0 |
| init | 0.000015 | 0.000000 | 0.000000 | 0 | 0 |
| System lock | 0.000007 | 0.000000 | 0.000000 | 0 | 0 |
| optimizing | 0.000003 | 0.000000 | 0.000000 | 0 | 0 |
| statistics | 0.000011 | 0.000000 | 0.000000 | 0 | 0 |
| preparing | 0.000008 | 0.000000 | 0.000000 | 0 | 0 |
| executing | 0.000002 | 0.000000 | 0.000000 | 0 | 0 |
| Sending data | 0.256462 | 0.256000 | 0.000000 | 0 | 0 |
| end | 0.000010 | 0.000000 | 0.000000 | 0 | 0 |
| query end | 0.000005 | 0.000000 | 0.000000 | 0 | 0 |
| closing tables | 0.000004 | 0.000000 | 0.000000 | 0 | 0 |
| freeing items | 0.000009 | 0.000000 | 0.000000 | 0 | 0 |
| cleaning up | 0.000010 | 0.000000 | 0.000000 | 0 | 0 |
+———————-+———-+———-+————+————–+—————+
```

可以查看到其每个步骤所要的时间，比如optimizing表示优化器优化时间等，这里就可以看到一条sql的完整生命周期。一般主要关注点有4个，如果出现就需要注意并优化：

-   converting HEAP to MyISAM:查询结果大，内存不够用了往磁盘上搬。
-   Creating tmp table:创建临时表
-   Copying to tmp table on
    disk:把内存中临时表复制到磁盘，一定非常耗费内存，危险。
-   locked

那么如何优化，这个时候就可以采用explain + sql的形式，进行细致的优化。

还有一种日志方式叫全局查询日志，该方法会将所有的sql语句保存下来，如果出现bug，可以查看从头到尾执行的sql语句，手动复现这些语句，更细致的查找到哪条语句导致的bug，生产环境禁止使用，一般在测试环境使用，具体使用方式请百度。
