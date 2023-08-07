mod handler;

use super::{get, list, random, remove, save};
use crate::common::IntoDocument;
use chrono::prelude::*;
pub use handler::*;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wish {
    #[serde(serialize_with = "format_oid")] // 序列化后的格式
    _id: Option<ObjectId>,
    pub author: Option<String>,      // 作者
    pub des: Option<String>,         // 心愿
    pub create_time: Option<String>, // timestamp
}

impl Wish {
    pub const TABLE_NAME: &'static str = "wish";

    #[allow(dead_code)]
    pub fn new() -> Self {
        Wish {
            _id: None,
            author: None,
            des: None,
            create_time: Some(Local::now().timestamp().to_string()),
        }
    }

    pub fn update_time(&mut self) {
        self.create_time = Some(Local::now().timestamp().to_string());
    }
}

impl IntoDocument<'_> for Wish {}

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
