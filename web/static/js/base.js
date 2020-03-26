baseUrl = "http://localhost:80";
pageUrl = "http://localhost:8080";
homeUrl = pageUrl + "/index.html";
owner = "qinxiaoguang";

$(function () {
    $("#button_blog").click(function () {
        location.href = pageUrl + "/article/list.html";
    });

    $("#button_home").click(function () {
        location.href = pageUrl + "/index.html"
    })
    if (is_owner()) {
        // 在左下角添加 直连edit_list的页面
        var edit_list_url = pageUrl + "/article/edit_list.html";
        var edit_list_btn = '<span id = "edit_list_btn">' +
            '<a href="' + edit_list_url + '"> <i class = "fa fa-th-list"></i></a>' +
            '</span>';
        $("body").append($(edit_list_btn));
    }
});

function alert(msg) {
    $("#alert").remove();
    setTimeout(function () {
        var alert_primary = '<div id="alert" class="p-3 mb-2 bg-dark text-white right_bottom_alert">' +
            msg + '</div>';
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
    var alert_warning = '<div id="alert" class="p-3 mb-2 bg-danger text-white right_bottom_alert">' +
        msg + '</div>';
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
    var today = new Array(
        "Sun",
        "Mon",
        "Tue",
        "Wed",
        "Thu",
        "Fri",
        "Sat"
    );
    var hour = resize2two(d.getHours());
    var minutes = resize2two(d.getMinutes());
    var seconds = resize2two(d.getSeconds());
    return year + "-" + month + "-" + day + " " + hour + ":" +
        minutes;
}

function resize2two(num) {
    return num < 10 ? "0" + num : "" + num;
}

function remove_article(axios, artilce_id, success, error) {
    axios.delete(baseUrl + "/admin/article/" + artilce_id).then(success).catch(error);
}