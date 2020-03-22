// 生成session_id
pub fn generate_session_id(username: &str) -> Option<String> {
    let uid = uuid::Uuid::new_v4();
    Some(format!("{}_{:?}", username, uid))
}

mod test {
    #[test]
    fn test_sid() {
        let sid = super::generate_session_id("username").unwrap();
        println!("{}", sid);
    }
}
