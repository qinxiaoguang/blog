use super::Quote;
use crate::common::*;
use bson::{doc, Document};
use mongodb::options::FindOptions;

pub fn save_quote(quote: Quote) -> Result<String> {
    super::save(Quote::TABLE_NAME, quote)
}

// base 获取某个db_article
#[allow(dead_code)]
pub fn get_quote(id: &str) -> Result<Quote> {
    super::get(Quote::TABLE_NAME, id)
}

#[allow(dead_code)]
// base 列出指定文章
pub fn list_quotes(filter: Option<Document>, find_options: FindOptions) -> Result<Vec<Quote>> {
    super::list(Quote::TABLE_NAME, filter, find_options)
}

#[allow(dead_code)]
pub fn remove_quote(id: &str) -> Result<i64> {
    super::remove(Quote::TABLE_NAME, id)
}

#[allow(dead_code)]
pub fn list_all_quote() -> Result<Vec<Quote>> {
    let find_options = FindOptions::builder().build();
    let filter = Some(doc! {});
    list_quotes(filter, find_options)
}

pub fn get_random_quote() -> Result<Quote> {
    super::random(Quote::TABLE_NAME)
}

mod test {
    #[test]
    fn test_get_rand() {
        println!("{:?}", get_random_quote());
    }

    #[test]
    fn test_save_quote() {
        let mut quote = Quote::new();
        quote.des= Some("慢慢发现自己其实是一个冷淡的人，所有情绪藏着掖着，变成了以后殆尽的热情。离开一个地方，就不会想和过去的还有联系，即使是非常喜欢的人，想明白也就放下了。".to_string());
        quote.author = Some("德卡先生".to_string());
        save_quote(quote).unwrap();
    }
}
