use crate::{
    common::{BizError, CommonResp, Resp},
    GLOBAL_CONF,
};
use actix_multipart::Multipart;
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};
use image::{imageops::FilterType, GenericImageView};
use log::info;

const PIC_SAVE_PATH: &str = "/static/img/upload/";

// 上传图片
pub async fn upload_pic(mut payload: Multipart) -> CommonResp {
    // iterate over multipart stream
    let mut gen_filename = uuid::Uuid::new_v4().to_simple().to_string();
    let mut filepath = String::from("");
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| BizError::UploadError)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| BizError::UploadError)?;
        match get_suffix(filename) {
            Some(suffix) => {
                gen_filename = gen_filename + "." + suffix;
            }
            None => {
                return Resp::err(crate::common::UPLOAD_ERROR, "upload failed").to_json();
            }
        };
        filepath = format!(
            "{}{}{}",
            &GLOBAL_CONF.server.web_path.clone().unwrap(),
            PIC_SAVE_PATH,
            gen_filename
        );
        info!("upload file path is:{}", filepath);
        let mut f = async_std::fs::File::create(filepath.clone()).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    // 写完毕后，将对应的图片进行压缩,压缩为600大小的
    resize_img(&filepath.clone());
    Resp::ok(gen_filename).to_json()
}

// 获取文件的suffix,suffix只能是png,jpg,jpeg等几种
fn get_suffix(filename: &str) -> Option<&str> {
    let allow_ends = vec!["png", "jpg", "jpeg"];
    for suffix in allow_ends.into_iter() {
        if filename.ends_with(suffix) {
            return Some(suffix);
        }
    }
    return None;
}

// 压缩图片
fn resize_img(filepath: &str) {
    let img = image::open(filepath.clone()).unwrap();
    let (width, height) = img.dimensions();
    let newsize = 600;
    if width >= newsize || height >= newsize {
        //let newimg = img.resize(800, 600 * height / width, FilterType::Lanczos3);
        let newimg = img.resize(newsize, newsize, FilterType::Lanczos3);
        let _ = newimg.save(filepath);
        //img.save(filepath).unwrap();
    }
}

mod test {
    #[test]
    fn test_resize_img() {
        super::resize_img("output/test.png")
    }
}
