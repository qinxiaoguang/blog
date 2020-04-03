use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

// 获取某个文件的内容，若文件为空，则创建
fn get_content_or_create(filename: String) -> String {
    let mut file = match OpenOptions::new().append(true).open(&filename) {
        Err(_) =>
        // 打开失败，则创建
        {
            File::create(&filename).unwrap()
        }
        Ok(f) => f,
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

fn save_to_file(filename: String, content: String) -> Result<String, String> {
    let mut file = match File::open(&filename) {
        Err(_) =>
        // 打开失败，则创建
        {
            File::create(&filename).expect("create file error")
        }
        Ok(f) => f,
    };
    file.write_all(content.as_bytes())
        .expect("write file error");
    Ok("write success".to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_get_content() {
        let content = get_content_or_create("tmpfile/qxg".to_string());
        println!("content is :{}", content);
    }
}
