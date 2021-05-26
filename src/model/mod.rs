pub mod article;
pub mod media;
pub mod quote;
pub mod wish;
use crate::common::IntoDocument;
use crate::common::*;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::{CountOptions, FindOneOptions, FindOptions};
use rand::prelude::*;

// 保存文章，返回文章id

// 用户创建文章的时候，先调用这个接口，其中的title和content都先是默认值，然后将页面跳转到alter.html页面
pub fn save<'a, T>(table_name: &str, model: T) -> Result<String>
where
    T: IntoDocument<'a> + ?Sized,
{
    let mut d = bson::to_bson(&model)
        .map(|x| x.as_document().unwrap().to_owned())
        .unwrap();
    d.remove("_id");
    match table(table_name).insert_one(d, None) {
        Ok(rs) => {
            let new_id = rs.inserted_id.as_object_id().expect("cant find object_id");
            // unwrap如果错误，则panic,而\?则比较聪明，如果unwrap失败则直接return err,不会panic,否则返回unwrap之后的值。
            Ok(new_id.to_string())
        }
        Err(e) => Err(e.into()),
    }
}

// selectById
pub fn get<'a, T>(table_name: &str, id: &str) -> Result<T>
where
    T: IntoDocument<'a> + ?Sized,
{
    let filter = Some(doc! {"_id" => ObjectId::with_string(id)?});
    let document = table(table_name)
        .find_one(filter, None)
        .expect("cant find this");
    Ok(bson::from_bson(bson::Bson::Document(document.unwrap()))?)
}

// 列出指定model
pub fn list<'a, T>(
    table_name: &str,
    filter: Option<Document>,
    find_options: FindOptions,
) -> Result<Vec<T>>
where
    T: IntoDocument<'a> + ?Sized,
{
    let cursor = table(table_name).find(filter, Some(find_options));
    cursor.map(|mut x| x.to_vec::<T>()).map_err(|e| e.into())
}

// return : 返回更改的个数
// update比较特殊，可能需要对某些字段做特殊处理，所以使用document的方式
pub fn update(table_name: &str, id: &str, document: Document) -> Result<i64> {
    let filter = doc! {"_id" => ObjectId::with_string(id)?};
    let update = doc! {"$set":document};
    Ok(table(table_name)
        .update_one(filter, update, None)
        .map(|x| x.modified_count)?)
    // 也可以这么写
    // table(xx).update_one(xx).map(xxx).map_err(|e|e.into())
}

// 删除对应的数据
pub fn remove(table_name: &str, id: &str) -> Result<i64> {
    let filter = doc! {"_id" => ObjectId::with_string(id)?};
    Ok(table(table_name)
        .delete_one(filter, None)
        .map(|x| x.deleted_count)?)
}

pub fn count_by(table_name: &str, filter: Option<Document>, options: CountOptions) -> Result<i64> {
    table(table_name)
        .count_documents(filter, Some(options))
        .map_err(|e| e.into())
}

// 获取总记录条数
pub fn count(table_name: &str) -> Result<i64> {
    let filter = doc! {};
    let options = CountOptions::default();
    table(table_name)
        .count_documents(filter, options)
        .map_err(|e| e.into())
}

pub fn random<'a, T>(table_name: &str) -> Result<T>
where
    T: IntoDocument<'a> + ?Sized,
{
    let count = count(table_name)?;
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(0, count);
    let find_options = FindOneOptions::builder().skip(rand_num).build();
    let filter = Some(doc! {});
    let cursor = table(table_name).find_one(filter, Some(find_options));
    cursor
        .map(|item| {
            let doc = item.unwrap();
            let bson = bson::Bson::Document(doc);
            bson::from_bson(bson).unwrap()
        })
        .map_err(|e| e.into())
}

mod test {
    use super::*;
    #[test]
    fn test_count() {
        println!("{:?}", count("article"));
    }
}
