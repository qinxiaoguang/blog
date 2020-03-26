// 生成session_id
const SPLIT_PATTERN: &str = "_";
pub fn generate_session_id(username: &str) -> Option<String> {
    let uid = uuid::Uuid::new_v4();
    Some(format!("{}{}{:?}", username, SPLIT_PATTERN, uid))
}

pub fn get_username(sid: &str) -> Option<String> {
    let res = sid.split(SPLIT_PATTERN).collect::<Vec<&str>>();
    if res.is_empty() {
        return None;
    }
    Some(res[0].to_owned())
}

mod test {
    #[test]
    fn test_sid() {
        let sid = super::generate_session_id("username").unwrap();
        println!("{}", sid);
    }
}
