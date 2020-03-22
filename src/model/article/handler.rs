use super::Article;
use crate::common::*;
use bson::doc;
use bson::oid::ObjectId;
use log::*;
use mongodb::options::FindOptions;

// 保存文章，返回文章id
pub fn save_article(article: Article) -> Result<String> {
    // 这里的map并不是iter的map，Rust中Result和Option都有map方法，
    // 是将Result<T> or Option<T> 转换为 Result<U> or Option<U>
    // 而and_then和map的效果一样，只不过and_then的fn返回的是Result或Option，所以and_then中返回的数据可能是错误的或者空的
    let mut d = bson::to_bson(&article)
        .map(|x| x.as_document().unwrap().to_owned())
        .unwrap();
    d.remove("_id");
    match table(Article::TABLE_NAME).insert_one(d, None) {
        Ok(rs) => {
            let new_id = rs.inserted_id.to_string();
            // unwrap如果错误，则panic,而\?则比较聪明，如果unwrap失败则直接return err,不会panic,否则返回unwrap之后的值。
            info!("article is : {:?}", article);
            Ok(new_id)
        }
        Err(e) => {
            error!("save article error : {}", e);
            Err(BizError::InternalError)
        }
    }
}

// 获取某个article
pub fn get_article(id: &str) -> Result<Article> {
    let filter = Some(doc! {"_id" => ObjectId::with_string(id)?});

    let document = table(Article::TABLE_NAME)
        .find_one(filter, None)
        .expect("cant find this");
    Ok(bson::from_bson(bson::Bson::Document(document.unwrap()))?)
}

// 列出所有文章
pub fn list_all_articles() -> Result<Vec<Article>> {
    let find_options = FindOptions::builder().build();
    list_articles(find_options)
}

// 列出最近的n篇文章
pub fn list_recent_articles(num: i64) -> Result<Vec<Article>> {
    let find_options = FindOptions::builder().limit(num).build();
    list_articles(find_options)
}

// 列出指定文章
pub fn list_articles(find_options: FindOptions) -> Result<Vec<Article>> {
    let cursor = table(Article::TABLE_NAME).find(Some(doc! {}), Some(find_options));
    cursor
        .map(|mut x| x.to_vec::<Article>())
        .map_err(|e| e.into())
}

// 更新id对应的文章
// return : 返回更改的个数
pub fn update_article(id: &str, article: Article) -> Result<i64> {
    println!("article is :{:?}", article);
    let filter = doc! {"_id" => ObjectId::with_string(id)?};
    let update = doc! {"$set":article.to_document().unwrap()};
    Ok(table(Article::TABLE_NAME)
        .update_one(filter, update, None)
        .map(|x| x.modified_count)?)

    // 也可以这么写
    // table(xx).update_one(xx).map(xxx).map_err(|e|e.into())
}

// 删除对应的文章
pub fn remove_article(id: &str) -> Result<i64> {
    let filter = doc! {"_id" => ObjectId::with_string(id)?};
    Ok(table(Article::TABLE_NAME)
        .delete_one(filter, None)
        .map(|x| x.deleted_count)?)
}

mod test {
    #[test]
    fn test_list_article() {
        println!("{:?}", super::get_article("5dca1c99008bb1bb000709fd"));
    }
}
