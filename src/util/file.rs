use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

// 获取某个文件的内容，若文件为空，则创建
pub fn get_content_or_create(filename: String) -> String {
    let mut file = match OpenOptions::new().read(true).open(&filename) {
        Err(_) => {
            File::create(&filename).expect("create failed");
            return "".to_string();
        }
        Ok(f) => f,
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

pub fn save_to_file(filename: String, content: String) -> std::io::Result<String> {
    File::create(&filename)?
        .write_all(content.as_bytes())
        .map(|_| "write success".to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_get_content() {
        let content = get_content_or_create("tmpfile/qxg".to_string());
        println!("content is :{}", content);
    }

    #[test]
    pub fn test_save_content() {
        println!(
            "{:?}",
            save_to_file("tmpfile/qxg".to_string(), "gagha".to_string())
        );
    }
}
