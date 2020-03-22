mod handler;
pub use handler::*;

use crate::common::IntoDocument;
use bson::oid::ObjectId;
pub use handler::*; // 把handler的内容才这个地方导出去，外部直接可以通过article::来进行调用
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    #[serde(serialize_with = "format_oid")] // 序列化后的格式
    _id: Option<ObjectId>,
    title: String,
    content: String,
}

// TODO
// 保存版本控制下的老文章,每次更新时都会把当前文章保存到该目录
#[derive(Debug, Serialize, Deserialize)]
pub struct OldArticle {
    article_id: String,
    // 第一个文章内容，第二个是版本信,扩展项为创建时间
    content: Vec<(String, String)>,
    // create_time: xxx
}

impl IntoDocument<'_> for Article {}

impl Article {
    pub const TABLE_NAME: &'static str = "article";
}

pub fn format_oid<S>(oid: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // as_ref是将Option<T>转换为Option<&T>
    match oid.as_ref().map(|x| x.to_hex()) {
        Some(v) => s.serialize_str(&v),
        None => s.serialize_none(),
    }
}
