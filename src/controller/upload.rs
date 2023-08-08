use std::path::Path;

use crate::{
    common::{BizError, CommonResp, Resp, Result},
    GLOBAL_CONF,
};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use image::{imageops::FilterType, GenericImageView};
use log::info;
use tokio::{fs, io::AsyncWriteExt};

const PIC_SAVE_PATH: &str = "/static/img/upload/";

// 上传图片
pub async fn upload_pic(mut payload: Multipart) -> CommonResp {
    // iterate over multipart stream
    let mut gen_filename = uuid::Uuid::new_v4().to_simple().to_string();
    let mut filepath = String::from("");
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();
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
        let mut f = tokio::fs::File::create(filepath.clone()).await?;
        //let mut f = async_std::fs::File::create(filepath.clone()).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
        f.flush().await?;
        drop(f);
    }
    // 写完毕后，将对应的图片进行压缩,压缩为600大小的
    tokio::spawn(compress_img(filepath.clone()));
    //compress_img(&filepath.clone()).await?;
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

#[allow(dead_code)]
fn resize_img(filepath: &str) -> Result<()> {
    let img = image::open(filepath.clone())?;
    let (width, height) = img.dimensions();
    let newsize = 800;
    if width >= newsize || height >= newsize {
        //let newimg = img.resize(800, 600 * height / width, FilterType::Lanczos3);
        let newimg = img.resize(newsize, newsize, FilterType::Lanczos3);
        let _ = newimg.save(filepath);
        //img.save(filepath).unwrap();
    }
    Ok(())
}

// 压缩图片
async fn compress_img(filepath: String) -> Result<()> {
    let filepath = &filepath;
    let ext = Path::new(filepath)
        .extension()
        .map(|x| x.to_str().unwrap_or("png"))
        .unwrap_or("png");
    let img = image::open(filepath.clone())?;
    let bak_filepath = &format!("{filepath}.bak.{ext}");
    //println!("{}", bak_filepath);
    let before_bytes = tokio::fs::metadata(filepath).await?.len();
    //println!("before bytes is:{}", before_bytes);
    let (width, height) = img.dimensions();
    let newimg = img.resize(width, height, FilterType::Lanczos3);
    // bak_filepath
    let _ = tokio::fs::File::create(bak_filepath).await?;
    let _ = newimg.save(bak_filepath);
    let after_bytes = tokio::fs::metadata(bak_filepath).await?.len();
    //println!("bytes is:{}", after_bytes);

    if before_bytes > after_bytes {
        // 后替换前
        fs::remove_file(filepath).await?;
        fs::rename(bak_filepath, filepath).await?;
    } else {
        // 删除后者
        fs::remove_file(bak_filepath).await?;
    }
    Ok(())
}

mod test {
    #[tokio::test]
    async fn test_resize_img() -> std::io::Result<()> {
        super::compress_img("output/test.jpg").await;
        Ok(())
    }
}
