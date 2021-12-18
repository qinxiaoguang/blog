document.write('<script src="https://cdn.bootcss.com/axios/0.19.2/axios.min.js"></script>');

// 测试环境
baseUrl = "http://${ip}:8080";
pageUrl = "http://${ip}";
// 线上环境
// baseUrl = "http://qxgzone.com:8080";
// pageUrl = "http://qxgzone.com";
homeUrl = pageUrl + "/index.html";
owner = "qinxiaoguang";
bPlayerUrl = "http://player.bilibili.com/player.html?";

document.write('<script src="/static/js/base_func.js"></script>');
