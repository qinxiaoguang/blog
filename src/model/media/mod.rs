mod handler;
use super::{get, list, remove, save, update};
use crate::common::IntoDocument;
use bson::oid::ObjectId;
use chrono::prelude::*;
pub use handler::*;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Media {
    #[serde(serialize_with = "format_oid")] // 序列化后的格式
    _id: Option<ObjectId>,
    name: Option<String>, // 名称
    des: Option<String>,  // 描述
    url: Option<String>,
    media_type: Option<i32>,     // 默认是图片
    create_time: Option<String>, // 创建时间
}

impl Media {
    const IMAGE_TYPE: i32 = 1;
    const VIDEO_TYPE: i32 = 2;
    pub const TABLE_NAME: &'static str = "article";

    pub fn new() -> Self {
        Media {
            _id: None,
            name: None,
            des: None,
            url: None,
            media_type: None,
            create_time: Some(Local::now().timestamp().to_string()),
        }
    }
}

impl IntoDocument<'_> for Media {}

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
