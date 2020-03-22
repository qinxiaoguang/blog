use crate::common::redis_helper;
// 根据sid判断是否登录
// 返回true表示登录
pub fn is_login(sid: &str) -> bool {
    !redis_helper::get_string(sid).is_none()
}
