use crate::common::redis_helper;
use crate::common::session_helper;
// 根据sid判断是否登录
// 返回true表示登录
pub fn is_login(sid: &str) -> bool {
    !redis_helper::get_string(sid).is_none()
}

pub fn is_owner(sid: &str) -> bool {
    if let Some(username) = session_helper::get_username(sid) {
        return username == "qinxiaoguang";
    }
    false
}
