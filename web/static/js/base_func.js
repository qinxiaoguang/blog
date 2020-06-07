CODE_NEED_LOGIN = 10003;
loginUrl = pageUrl + "/login.html";

$(function () {
    $("#button_blog").click(function () {
        location.href = pageUrl + "/article/list.html";
    });

    $("#button_home").click(function () {
        location.href = pageUrl + "/index.html";
    });

    $("#button_life").click(function () {
        location.href = pageUrl + "/life/list.html";
    });

    $("#button_about").click(function () {
        location.href = pageUrl + "/about.html";
    });

    $("#button_tool").click(function () {
        location.href = pageUrl + "/tool/index.html";
    });

    if (is_owner()) {
        // 在右下角添加 直连edit_list的页面
        var edit_list_url = pageUrl + "/article/edit_list.html";
        var edit_list_btn =
            '<span id = "edit_list_btn">' +
            '<a href="' +
            edit_list_url +
            '"> <i class = "fa fa-th-list"></i></a>' +
            "</span>";
        $("body").append($(edit_list_btn));
    }

    // 获取点击次数
    knock();

    // 给body添加统一loading组件
    let loading = '<div id="loading"></div>';
    $("body").append(loading);
    
    // 如果不是phone的话，添加音乐播放器
    // 只在index页面和life/list页面添加
    if (!is_phone() && (location.pathname == "/index.html" || location.pathname == "/life/list.html" || location.pathname == "/" )) {
        let music = '<div id="aplayer"></div>';
        $("body").append(music);

	// 添加完毕后，获取音乐列表
        getMusicList()
    }
});

/*function imgBigShow(target) {
    if (target.classList.contains("imgBigShow")) {
        target.classList.remove("imgBigShow");
    } else {
        target.classList.add("imgBigShow");
    }
}*/

$(document).scroll(scrollHandler); //监听屏幕滚动事件，保证header始终在最上方

$(window).resize(function () { //当浏览器大小变化时
    // 监听屏幕resize事件，手机的resize时，调用fit_code来自适应 code
    fit_code();
});

var screen = $(window).width(); // 屏幕宽度
var header = $('#header');
var headerHeight = 100;
var headerHeightPx = headerHeight + "px";
var phoneMenuHeight = 56;
var phoneMenuHeightPx = phoneMenuHeight + "px";
var scrollHeight = headerHeight - phoneMenuHeight;

function is_phone() {
    return screen < 720;
}

function scrollHandler(e) {
    // 视差滚动，当屏幕小于720px时，滑动设置header为fixed
    if (is_phone()) {
        var nowTop = document.documentElement.scrollTop || document.body.scrollTop;
        if (nowTop >= scrollHeight) {
            //如果当前滑动的值大于100则，则将nav的position变为fixed
            header.css("position", "fixed")
            header.css("top", "0px")
            header.css("height", phoneMenuHeightPx)
        } else {
            header.css("position", "relative")
            header.css("top", "0px")
            header.css("height", headerHeightPx)
        }
    }
}

var knockCnt = 0;
var knockRefreshDone = false;

function loading_show() {
    $("#loading").css("display", "block");
    $("#main").css("display", "none");
    $(".main").css("display", "none");
}

function loading_finish() {
    $("#loading").css("display", "none");
    $("#main").css("display", "block");
    $(".main").css("display", "block");
    $("#main").addClass("fadeIn");
    $(".main").addClass("fadeIn");
}


function knock() {
    axios
        .get(baseUrl + "/knock")
        .then(response => {
            if (response.data.code == 0) {
                knockCnt = response.data.data;
            }
            knockRefreshDone = true;
        }).catch(err => {
            knockRefreshDone = true;
        })
}

function getKnockCnt(success) {
    var i = setInterval(function () {
        if (knockRefreshDone) {
            success(knockCnt);
            // 停掉interval
            clearInterval(i);
        }
    }, 500);
}

function alert(msg) {
    $("#alert").remove();
    setTimeout(function () {
        var alert_primary =
            '<div id="alert" class="p-3 mb-2 bg-dark text-white right_bottom_alert">' +
            msg +
            "</div>";
        $("body").append($(alert_primary));
        setTimeout(function () {
            $("#alert").remove();
        }, 4000);
    }, 50);
}

function alert_refresh(msg) {
    $("#alert").html(msg);
}

function warn(msg) {
    $("#alert").remove();
    var alert_warning =
        '<div id="alert" class="p-3 mb-2 bg-danger text-white right_bottom_alert">' +
        msg +
        "</div>";
    $("body").append($(alert_warning));
    setTimeout(function () {
        $("#alert").remove();
    }, 3000);
}

// 判断是否登录
function is_owner() {
    return document.cookie.indexOf("sessionid=" + owner) !== -1;
}

function format_date(timestamp) {
    var d = new Date(timestamp * 1000);
    var year = d.getFullYear();
    var month = resize2two(d.getMonth() + 1);
    var day = resize2two(d.getDate());
    var week = d.getDay();
    var today = new Array("Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat");
    var hour = resize2two(d.getHours());
    var minutes = resize2two(d.getMinutes());
    var seconds = resize2two(d.getSeconds());
    return year + "-" + month + "-" + day + " " + hour + ":" + minutes;
}

function resize2two(num) {
    return num < 10 ? "0" + num : "" + num;
}

function remove_article(axios, artilce_id, success, error) {
    axios
        .delete(baseUrl + "/admin/article/" + artilce_id)
        .then(success)
        .catch(error);
}

function remove_media(axios, media_id, success, error) {
    axios
        .delete(baseUrl + "/admin/media/" + media_id)
        .then(success)
        .catch(error);
}

function upload_media(axios, data, success, error) {
    axios
        .post(baseUrl + "/admin/media", data)
        .then(success)
        .catch(error);
}

function get_img_url(path) {
    return pageUrl + path;
}

function get_video_url(path) {
    return bPlayerUrl + path;
}

function base_upload_pic(axios, file, success, error) {
    if (
        file.name.indexOf("jpg") === -1 &&
        file.name.indexOf("jpeg") === -1 &&
        file.name.indexOf("png") === -1
    ) {
        alert("只能上传jpg, jpeg,png等格式内容");
    }
    var file_size = file.size / 1024; // kb
    if (file_size >= 10 * 1024) {
        alert("上传文件过大");
    }
    var formData = new FormData();
    formData.append("file", file);
    let config = {
        headers: {
            "Content-Type": "multipart/form-data",
        },
    };
    axios
        .post(baseUrl + "/admin/upload/pic", formData, config)
        .then(success)
        .catch(error);
    // 上传
}

function check_login(data) {
    console.log(data)
    if (data.code == CODE_NEED_LOGIN) {
        location.href = loginUrl;
    }
}

// 自适应高度
function fit_height() {
    var screenHeight = document.documentElement.clientHeight;
    var bodyHeight = document.body.clientHeight;
    console.log(bodyHeight + "," + screenHeight);
    if (bodyHeight < screenHeight) {
        $("#bottom_").css("position", "fixed");
        $("#bottom_").css("bottom", "0px");
        $("#main").css("height", screenHeight);
    } else {
        $("#bottom_").css("position", "relative");
        $("#bottom_").css("bottom", '');
        $("#main").css("height", '');
    }
}

// 手机code自适应宽度
function fit_code() {
    if (is_phone() && location.pathname == "/article/detail.html") {
        // 先把content pre的宽度设置为0，再查看contentWidth
        $("#content pre").css("width", "0");
        contentWidth = $("#content")[0].clientWidth;
        // 再将content pre设置为该值
        $("#content pre").css("width", contentWidth);
    }
}

function toc_init() {
    $("body").click(function (e) {
        $("#toc").css("display", "none");
        $("body").css("width", "");
        $("body").css("float", "");
    });
    $("body").keydown(function (e) {
        if (e.keyCode == 9) {
            var toc_display = $("#toc").css("display");
            if (toc_display == "block") {
                $("#toc").css("display", "none");
                $("body").css("width", "");
                $("body").css("float", "");
            } else {
                $("#toc").css("display", "block");
                $("body").css("width", "85%");
                $("body").css("float", "right");
            }
            e.preventDefault();
        }
    });
}

function getMusicList(){
     axios
        .get(baseUrl + "/music/list/5047601141")
        .then(response => {
            var audios = [];
            for (item of response.data.data){
		console.log(item)
		var audio = {
		    "name":item.name,
		    "url":item.url,
		    "cover":item.cover,
		    "artist":item.artist,
		}
		audios.push(audio)
	    }
	    console.log(audios)
            const ap = new APlayer({
                container: document.getElementById('aplayer'),
                audio:audios,
		theme: '#f9ed69',
		mutex: true,
		autoplay: true,
		order: 'random', 
		fixed:true,
            });
            console.log("get_music done");
        }).catch(err => {
	    console.log("music load failed, err is:"+err);
        })
}
