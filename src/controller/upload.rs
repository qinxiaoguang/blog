use crate::common::{BizError, CommonResp, Resp};
use actix_multipart::Multipart;
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};

const PIC_SAVE_PATH: &str = "web/static/img/upload/";

// 上传图片
pub async fn upload_pic(mut payload: Multipart) -> CommonResp {
    // iterate over multipart stream
    let mut gen_filename = uuid::Uuid::new_v4().to_simple().to_string() + ".jpg";
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| BizError::UploadError)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| BizError::UploadError)?;
        if !check_pic(filename) {
            return Resp::err(crate::common::UPLOAD_ERROR, "upload failed").to_json();
        }
        let filepath = format!("{}{}", PIC_SAVE_PATH, gen_filename);
        println!("file path is:{}",filepath);
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Resp::ok(gen_filename).to_json()
}

// 检查文件名后缀
fn check_pic(filename: &str) -> bool {
    let allow_ends = vec!["png", "jpg", "jpeg"];
    allow_ends.iter().any(|end| filename.ends_with(end))
}
