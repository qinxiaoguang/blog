mod handler;
pub use handler::*;

use crate::common::IntoDocument;
use bson::oid::ObjectId;
use chrono::prelude::*;
pub use handler::*; // 把handler的内容才这个地方导出去，外部直接可以通过article::来进行调用
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Article {
    #[serde(serialize_with = "format_oid")] // 序列化后的格式
    _id: Option<ObjectId>,
    title: Option<String>,
    content: Option<String>,
    catagory: Option<String>,    // 文章分类, 只能为一种种类
    tag: Option<Vec<String>>,    // 文章tag or关键词
    update_time: Option<String>, // 更新时间
}

impl Article {
    pub fn time_update(&mut self) {
        self.update_time = Some(Local::now().timestamp().to_string())
    }
}

type Summary = std::collections::BTreeMap<String, Vec<Article>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbArticle {
    #[serde(serialize_with = "format_oid")] // 序列化后的格式
    _id: Option<ObjectId>,
    publish: Option<Article>,
    edit: Option<Article>,
    status: Option<i32>,               // 默认状态 为未发布
    create_time: Option<String>,       // timestamp
    last_publish_time: Option<String>, // publish的更新时间，按此时间排序，展示最新文章
}

impl DbArticle {
    pub const TABLE_NAME: &'static str = "article";
    #[allow(dead_code)]
    pub const PUBLISHED: i32 = 1; // 发布状态，非作者也可见
    #[allow(dead_code)]
    pub const DEFAULT: i32 = 0; // 默认状态，未发布

    pub fn new(article: Article) -> Self {
        DbArticle {
            _id: None,
            edit: Some(article),
            publish: None,
            status: None,
            last_publish_time: None,
            create_time: Some(Local::now().timestamp().to_string()),
        }
    }

    pub fn into_publish(mut self) -> Option<Article> {
        if let Some(ref mut article) = self.publish {
            article._id = self._id.clone();
        }
        self.publish
    }

    #[allow(dead_code)]
    pub fn into_edit(mut self) -> Option<Article> {
        if let Some(ref mut article) = self.edit {
            article._id = self._id.clone();
        }
        self.edit
    }

    #[allow(dead_code)]
    pub fn is_publish(&self) -> bool {
        self.status.is_some() && self.status.unwrap() == Self::PUBLISHED
    }
}

// TODO 保存版本控制下的老文章,长期来说，此功能可能不需要
#[derive(Debug, Serialize, Deserialize)]
pub struct OldArticle {
    article_id: String,
    // 第一个文章内容，第二个是版本信,扩展项为创建时间
    content: Vec<String>,
    // create_time: xxx
}

impl OldArticle {
    // 缓存的老文章的最大长度
    #![allow(dead_code)]
    const CACHE_OLD_ARTICLE_MAX_LEN: usize = 5;
}

impl IntoDocument<'_> for OldArticle {}

impl IntoDocument<'_> for DbArticle {}

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
