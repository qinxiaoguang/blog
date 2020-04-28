\#\#+TITLE: vscode调试golang

概述
====

通过vscode来断点调试golang,方法是通过使用dlv插件。

方法
====

1.  安装dlv,注意保持golang在新版本，否则可能报错。
    `go get -u github.com/go-delve/delve/cmd/dlv`,
    如果安装失败，可以配置go代理:
    `go env -w  GOPROXY=https://goproxy.cn`
2.  在vscode中打开命令行输入: `Debug:Open launch.json`,
    他会在本地的.vscode目录中创建一个文件，该文件主要就是来配置在debug时候运行的命令及环境。默认会创建一个默认文件。
3.  配置文件为以下内容,内容解释上百度搜一下。

| {
|     // Use IntelliSense to learn about possible attributes.
|     // Hover to view descriptions of existing attributes.
|     // For more information, visit:
  <https://go.microsoft.com/fwlink/?linkid=830387>
|     \"version\": \"0.2.0\",
|     \"configurations\": \[{
|             \"name\": \"Launch\",
|             \"type\": \"go\",
|             \"request\": \"launch\",
|             \"mode\": \"auto\",
|             \"program\": \"\${workspaceFolder}\",
|             \"env\": {},
|             \"args\": \[\]
|         },
|         {
|             \"name\": \"Launch file\",
|             \"type\": \"go\",
|             \"request\": \"launch\",
|             \"mode\": \"auto\",
|             \"program\": \"\${file}\",
|             \"env\": {},
|             \"args\": \[\]
|         },
|         {
|             \"name\": \"Launch test function\",
|             \"type\": \"go\",
|             \"request\": \"launch\",
|             \"mode\": \"test\",
|             \"program\": \"\${workspaceFolder}\",
|             \"args\": \[
|                 \"-test.run\",
|                 \"MyTestFunction\"
|             \]
|         },
|         {
|             \"name\": \"Launch test package\",
|             \"type\": \"go\",
|             \"request\": \"launch\",
|             \"mode\": \"test\",
|             \"program\": \"\${workspaceFolder}\"
|         },
|         {
|             \"name\": \"Launch executable\",
|             \"type\": \"go\",
|             \"request\": \"launch\",
|             \"mode\": \"exec\",
|             \"program\": \"absolute-path-to-the-executable\"
|         }
|     \]
| }

1.  在项目路径上，按F5,进行调试，会自动打开dlv等插件进行调试。
