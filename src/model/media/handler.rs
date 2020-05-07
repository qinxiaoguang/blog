use super::Media;
use crate::common::*;
use bson::{doc, Document};
use mongodb::options::FindOptions;

pub fn save_media(mut media: Media) -> Result<String> {
    media.time_update();
    super::save(Media::TABLE_NAME, media)
}

// base 获取某个db_article
#[allow(dead_code)]
pub fn get_media(id: &str) -> Result<Media> {
    super::get(Media::TABLE_NAME, id)
}

// base 列出指定文章
pub fn list_medias(filter: Option<Document>, find_options: FindOptions) -> Result<Vec<Media>> {
    super::list(Media::TABLE_NAME, filter, find_options)
}

pub fn remove_media(id: &str) -> Result<i64> {
    super::remove(Media::TABLE_NAME, id)
}

pub fn list_all_media() -> Result<Vec<Media>> {
    let find_options = FindOptions::builder()
        .sort(Some(doc! {"create_time":-1}))
        .build();
    let filter = Some(doc! {});
    list_medias(filter, find_options)
}

#[allow(dead_code)]
pub fn list_recent_video(num: i64) -> Result<Vec<Media>> {
    list_recent(num, Media::VIDEO_TYPE)
}

pub fn list_recent_img(num: i64) -> Result<Vec<Media>> {
    list_recent(num, Media::IMAGE_TYPE)
}

pub fn list_recent_media(num: i64) -> Result<Vec<Media>> {
    list_recent(num, Media::NOT_NULL)
}

// 列出已经发布的n个media,media_type是0时，表示不分类
pub fn list_recent(num: i64, media_type: i32) -> Result<Vec<Media>> {
    let find_options = FindOptions::builder()
        .sort(Some(doc! {"create_time":-1}))
        .limit(num)
        .build();
    let filter = if media_type == Media::NOT_NULL {
        Some(doc! {})
    } else {
        Some(doc! {"media_type": media_type})
    };
    list_medias(filter, find_options)
}
