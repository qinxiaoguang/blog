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
        // 在左下角添加 直连edit_list的页面
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
});

var knockCnt = 0;
var knockRefreshDone = false;

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