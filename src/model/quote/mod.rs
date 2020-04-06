mod handler;
use super::{get, list, random, remove, save};
use crate::common::IntoDocument;
use bson::oid::ObjectId;
pub use handler::*;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    #[serde(serialize_with = "format_oid")] // 序列化后的格式
    _id: Option<ObjectId>,
    pub author: Option<String>, // 名称
    pub des: Option<String>,    // 描述
}

impl Quote {
    pub const TABLE_NAME: &'static str = "quote";

    pub fn new() -> Self {
        Quote {
            _id: None,
            author: None,
            des: None,
        }
    }
}

impl IntoDocument<'_> for Quote {}

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
