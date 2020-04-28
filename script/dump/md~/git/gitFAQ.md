忽略文件权限
============

有时候修改了文件的权限，都会与线上产生冲突，可以通过以下命令让git忽略权限
`git config core.filemode false`

git commit 提交后想要修改提交说明
=================================

`git commit --amend`
进入提交说明界面，修改原来的提交说明。或者说是覆盖之前的提交。如果提交评审后，发现有问题，再提交一遍评审就会出现两个记录，但是我想只有一条记录，方法就是使用git
commit --amend,注意只是用于评审过程中。

已提交的文件包含了不该提交的文件
================================

`git rm --cached xxx.xx` -\> `git commit --amend`

在未进行提交的时候切换分支
==========================

`git stash` -\> `git checkout <branch>` 切换回来后执行 `git stash pop`
`git stash` 是将工作区和暂存区的内容保存起来。

出现non-fast-forward
====================

如果确定代码没问题时，可以使用 `git push -f` 来强制覆盖

git 中HEAD的含义
================

HEAD是当前版本库的头指针，在 `.git/HEAD` 文件中有记录，如
`ref: refs/heads/master` ，表示的就是HEAD指向一个引用，该引用是
`refs/heads/master` 其保存在 `.git` 文件中，即 `.git/refs/heads/master`
。而master文件中则记录着一个提交的sha id，如
`e29f9aff771447d475d33a455340452e57659030` ，其内容是 `commit-id`
，可以使用 `git cat-file commit <commit-id>`
查看该commit的信息。注意HEAD是当前分支的头指针，其指向的就是当前分支，如果当前分支是qxg，那么就会在
`.git/refs/heads` 目录下生成 `qxg` 文件。 但使用 `git commit`
提交后，会生成一个新的sha1
id。同时HEAD指向的分支，如refs/heads/master将会指向最近的提交。
所以这里说明几个概念，当说工作区，暂存区，HEAD的时候，就可理解为三个不同的区域，
`git add` 就是将内容从工作去添加到暂存区中， `git commit`
就是将暂存区内容添加到HEAD新指向的区域。

有一点需要弄明白，当前在工作区所修改的内容，并不属于HEAD，HEAD是指当前已提交的版本。

git diff的不同效果
==================

`git diff` :该命令是工作区和暂存区的比较，即当前文件和 `git add`
后的文件的比较 `git diff --cached` :暂存区和HEAD的比较，即 `git add`
后的文件和 `git commit` 后的文件的比较。 `git diff HEAD`
:工作区和HEAD的比较，即当前工作区的内容和已提交的内容的比较。

git reset的用法
===============

假设当前分支位于 `refs/heads/master` 上，那么使用 `git reset`
可以将该分支的指向修改，或是修改 `refs/heads/master` 中的内容。
书中所说的 `refs/heads/master` 类似一个游标，但是当 `commit`
后，该游标只能往下走，如何往上走呢，方法就是使用 `git reset` 方法。但是
`git reset` 是比较危险的。

重置分支引用包括工作区和暂存区
------------------------------

以下方法的 `HEAD^` 都可替换为 `<commit-id>`

`git reset --hard HEAD^` ，包含 `--hard` 的 `git reset`
命令比较危险，原因是，该命令不仅仅会重置当前分支的指向，同时会将原有的暂存区的内容和工作区的内容清空，变为和当前重置后的分支一致。
假设在当前分支执行了 `git add somefile.txt` ,而在调用
`git reset --hard HEAD^` 后，暂存区中的 `somefile.txt` 就没有了。

`HEAD` 表示最近的一次提交， `HEAD^` 表示最近提交的父提交，使用
`git reset --hard HEAD^` 命令时，当前的内容会恢复到 `HEAD^`
刚提交的状态，或说当前的版本将变成 `HEAD^` 指定的版本。
其实再说明白一点，使用 `--hard`
的时候，工作区和commit-id对应的内容一摸一样。而之前工作区的内容就不见了，同时暂存区意味着被清空。

所以谨慎使用 `--hard`

重置分支的引用，不包括工作区和暂存区
------------------------------------

`git reset --soft HEAD^`
，该方法只是重置当前分支的引用，不会修改工作区和暂存区的内容。
当对最新的提交不满意的时候，或想修改提交的说明或部分文件时候，可以使用该命令，执行后，可继续进行操作之后再提交。

重置分支引用和暂存区但是不修改工作区
------------------------------------

`git reset --mixed HEAD^` ，默认是 `--mixed`

重置暂存区或清空暂存区
----------------------

`git reset` 或 `git reset HEAD` ，因为默认是 `--mixed`
，该命令会重置分支引用，但是因为重置的分支引用还是 `HEAD`
,所以可以看作没有重置，但是会重置暂存区。

清空工作区
----------

`git checkout .`

add文件后撤销
-------------

`git reset HEAD -- <file>` 或 `git reset HEAD <file>` 其中 `--`
是担心提交的id和文件产生冲突。 如果指定 `<file>` 或说是 `<path>` 时，
`reset` 则不是重置HEAD指向的master的引用。

或 `HEAD` 也可以省略，即 `git reset -- <file>`
，起到同样的作用，参看上一条。

git checkout 命令
=================

切换分支
--------

使用 `git checkout <branch-name>` 可以进行切换分支，而使用
`git chceckout <commit-id>` 则使HEAD直接指向 `<commit-id>`
，即其不指向某一分支，但是在这个地方提交后的内容虽然会生成一个新的commit-id,当切换一个有名字的分支，HEAD原先直接指向的commmit-id代表的内容将可能消失，因为没有任何一个分支对该内容进行追踪。但是如果没有消失，可以使用
`git merge` 命令将其内容合并到当前分支。

`git checkout -b <branch> <remote-name>/<branch>`
会创建分支并设置跟踪分支 而
`git checkout --track <remote-name>/<branch>`
会自动跟踪目标分支并创建一个分支与之对应。

如 `git checkout -b qxg origin/qxg` 和 `git checkout --track origin/qxg`
等价。

包含路径的git checkout命令
--------------------------

`git checkout -- <file>` 将暂存区中的文件取出覆盖工作区。如果刚刚使用
`git add` 提交某一文件，想要撤销可使用该命令，但是该命令是以覆盖的方式。

那么 `git checkout .` 或 `git checkout -- .`
则会将暂存区的所有文件取出并覆盖给工作区，意味着清空工作区内容

`git checkout <commit-id> -- <file>`
使用某提交后的文件来替换当前工作区和暂存区的文件。

git cherry-pick
---------------

该命令用于将某个commmit直接用作当前HEAD，比如当前分支位于master,存在另一个分支Test,现在想把Test中的内容整合到master，但是不是merge，因为还要在Test上进行操作，那么就可以使用cherry-pick

``` {.git}
git checkout master
git cherry-pick <Test>
```

该命令直接会完成一次提交，该提交和Test中的提交一摸一样包括内容和提交的说明。但是注意
`git cherry-pick` 只能把对应的最新版本提交到当前版本，和git
merge有很大出入。 `git merge` 成功后两个分支会变为一个。 `git rebase`
会将当前分支的所有commit都重定向到目标分支上，并整个分支都会消失，看似是一直在主分支上进行操作一样。

将test分支合并到master： `git checkout master` -\> `git merge test`
变基方式将test分支合并到master: `git checkout test` -\>
`git rebase master` ,如果遇到问题，解决后使用 `git add`
之后不用提交，直接 `git rebase --continue`

取消git add命令汇总
===================

-   `git reset`
-   `git reset --mixed`
-   `git reset HEAD`
-   `git reset --mixed HEAD`
-   `git checkout -- .` (保证git add后没有进行其他操作)
-   `git checkout .` (同上)

取消git add 某file命令汇总
==========================

-   `git checkout -- <file>`
-   `git checkout <file>`

取消git commit 到 git add 命令汇总
==================================

-   `git reset --soft HEAD^`

取消git commit到工作区命令汇总
==============================

-   `git reset --mixed HEAD^`

别名
====

-   `git config --system alias.lg "log --graph --pretty=oneline"`,之后
    `git lg` 就可以看到分支的运行状态。
-   `git config --system alias.st status` 则 `git st == git status`
-   善用别名

删除当前分支
============

`git branch -D <分支名>`,可能需要 `git stash` 命令

从指定远程分支pull代码到当前分支
================================

`git pull origin <branch-name>`

当pull或merge出现冲突时
=======================

使用 `git mergetool` 来解决冲突 或者自己修改冲突后，使用 `git add`
并提交即可。 如果不想修改冲突，可以使用 `git reset` 来取消暂存区的内容。

git revert
==========

`git revert HEAD`
所做的操作是撤销当前的提交，但是其并不是真正撤销，而是将对应的类似撤销的操作当作一个提交，提交后内容和上一次的提交内容一样。所以如果当前提交为
`A -> B` ，那么执行 `git revert HEAD` 后，当前的提交就变为
`A -> B -> A'` 其中 `A'` 和 `A` 内容一样，不过 `commit-id` 不一样。

git clone不仅克隆远程分支，也可克隆本地
=======================================

如: `git clone /home/qxg/demo /home/qxg/demo-backup`

git pull/push
=============

`git pull/push [<remote-repos> [<refspec>]]` \[\]标记的可省略，
`<remote-repose>` 是远程版本库， `refspec` 是引用表达式。

git remote
==========

`git remote -v`
可以查看上游版本库，正常操作就是设置的对应的远程版本库。其实注册的上游版本库在
`.git/config` 中都可以查看到。

git reflog
==========

当使用 `git reset` 来重置到之前的版本时，之间的版本将在 `git log`
中消失，如果想回退就需要使用 `git reflog` ，该命令打印出来的 `log`
会包含任何版本的信息，包括 `git reset` 的操作。

拉代码的时候让本地和远程对应
============================

`git checkout -b <new-branch> <remote-branch>` eg:
`git checkout -b qxg origin/master_xxx`

当执行 `git chekcout -b qxg`
来创建分支的时候，是不会指定远程跟踪分支的，也不会指定本地跟踪分支的。如果要创建远程跟踪分支，就是上边的命令。
`git checkout -b qxg <local-branch>`
执行该命令也不会将本地分支设置为跟踪分支，除非加上 `--track` ，即
`git checkout -b --track qxg <local-branch>` 。

如果没有设置跟踪分支
====================

如果没有设置对应的远程跟踪分支，或是没有设置上游分支，可以使用命令
`git branch -u <remote-name>/<branch>` 来设置，如
`git branch -u origin/qxg` 或 `git branch --set-upstream-to origin/qxg`
。

git tag
=======

给对应的commit-id创建一个标签，在其他需要commit-id的参数的时候，就可以用tag来代替。
`git tag <tag-name> <commit-id>` 如 `git tag super-power HEAD^`
表示给上一个版本的提交起个名字super-power

git status -s
=============

`git status -s` 或 `git status --short` 显示的status简略

| \$ git status -s
|  M README
| MM Rakefile
| A lib/git.rb
| M lib/simplegit.rb
| ?? LICENSE.txt

除了 `??`
表示未暂存，其他的标记只要在第一列都表示已加到暂存区了，第二列的M表示文件被修改，没加入暂存区，第一列的M表示文件被修改并加入暂存区，而第一列的A表示文件新添加的并加入了暂存区。

origin
======

origin是远程仓库的名字，当使用git pull 或git
push的时候，需要指定远程仓库，比如http://xxxxx.git这种，而origin默认就指定了这些仓库，所以
`git push origin qxg`
就是将当前的代码提交到远程仓库的qxg分支中，如果本地没有指定origin对应的仓库，则会执行失败。
同样的origin名字也可以更改，master也可以更改，master是分支。

当执行 `git push origin qxg` 时候，实际上是执行
`git push origin refs/heads/qxg:refs/heads/qxg` `:`
前表示本地分支，该代码就表示将本地的qxg分支推送到远程的qxg分支。
同样也可以使用 `git push origin qxg:qxg` ，而使用
`git push origin qxg:other` 将本地qxg分支推送到远程的other分支上。

git branch
==========

-   `git branch -a` 查看所有分支
-   `git branch -r` 查看远程分支
-   `git branch` 查看本地分支
-   `git branch -vv` 查看本地分支和远程分支的对应。

删除远程分支
============

-   `git push <remote-name> --delete <branch>`
-   `git push origin --delete qxg` 可以删除origin对应的qxg分支。

冲突的相关问题
==============

master分支pull完代码后合并到qxg分支出现冲突，解决完冲突后git add
,commit后，其实master和qxg就合并了，生成了一个新的节点并且该节点的父节点可以理解有两个，然后切换到master,git
merge
qxg其实就是将master指针直接指向那个交叉的节点。但是远程的master还在老位置，所以这个时候再pull代码不会有新代码了，这个时候master就可以提交了。仔细考虑考虑这个问题。

git push
========

-   默认的 `git push` 会把所有的分支都push。
-   如果想要执行特定的push,需要执行 `git push origin qxg`
    ,则会把qxg分支推送的远程的qxg分支上。

替换某个分支的文件到另一个分支
==============================

其实这个问题的解法有很多，一个是 `git cherry-pick`
，但是该方法的使用有局限，限制在某次提交上。 另一个是
`git checkout <commit-id> -- file` ,个人比较喜欢的是第二个。

git使用A账户commit,git push时报错，非本人操作
=============================================

此时应该使用 `git reset --soft HEAD^` ,该操作将状态还原到 `git add`
后，即重新进行提交，但是前提是已切换过账户。

.gitignore失效问题
==================

如果git
add将某些文件track了，那么此时再编写gitignore，只要这些文件被track就不会被忽略，此时需要做的操作是
`git rm -r --cached .` ,接着编写.gitignore再add,commit即可。

refs/for
========

refs/for不是git的规则，而是gerrit的规则，使用refs/for的需要进行评审，或叫做code
review才能进行合并。 refs/heads 则不需要code review,如上所述，
`git push origin master` 实际就是
`git push origin refs/heads/master:refs/heads/master` 的缩写

git log
=======

精简模式显示:
`git log --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit`
可以将其保存到系统的 `alias` 中,或者使用git config为其做个新的命名:
`git config --global alias.lg "log --color --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit"`
