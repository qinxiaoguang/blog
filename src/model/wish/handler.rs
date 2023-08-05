use super::Wish;
use crate::common::*;
use bson::{doc, Document};
use mongodb::options::FindOptions;

pub fn save_wish(wish: Wish) -> Result<String> {
    let mut wish = wish;
    wish.update_time();
    super::save(Wish::TABLE_NAME, wish)
}

#[allow(dead_code)]
pub fn get_wish(id: &str) -> Result<Wish> {
    super::get(Wish::TABLE_NAME, id)
}

#[allow(dead_code)]
pub fn list_wishs(filter: Option<Document>, find_options: FindOptions) -> Result<Vec<Wish>> {
    super::list(Wish::TABLE_NAME, filter, find_options)
}

#[allow(dead_code)]
pub fn remove_wish(id: &str) -> Result<i64> {
    super::remove(Wish::TABLE_NAME, id)
}

// 按时间倒序列出所有的愿望单
pub fn list_all_wish() -> Result<Vec<Wish>> {
    let find_options = FindOptions::builder()
        .sort(Some(doc! {"create_time":-1}))
        .build();
    let filter = Some(doc! {});
    list_wishs(filter, find_options)
}

pub fn get_random_wish() -> Result<Wish> {
    super::random(Wish::TABLE_NAME)
}

mod test {
    #[test]
    fn test_get_rand() {
        println!("{:?}", get_random_wish());
    }

    #[test]
    fn test_save_wish() {
        let mut wish = Wish::new();
        wish.des = Some("希望家人们永远安康".to_string());
        wish.author = Some("秦先生".to_string());
        save_wish(wish).unwrap();
    }
}
