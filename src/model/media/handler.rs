use super::Media;
use crate::common::*;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::FindOptions;
use std::collections::BTreeMap;

pub fn save_media(media: Media) -> Result<String> {
    super::save(Media::TABLE_NAME, media)
}

// base 获取某个db_article
pub fn get_media(id: &str) -> Result<Media> {
    super::get(Media::TABLE_NAME, id)
}

// base 列出指定文章
pub fn list_media(filter: Option<Document>, find_options: FindOptions) -> Result<Vec<Media>> {
    super::list(Media::TABLE_NAME, filter, find_options)
}

pub fn remove_media(id: &str) -> Result<i64> {
    super::remove(Media::TABLE_NAME, id)
}
