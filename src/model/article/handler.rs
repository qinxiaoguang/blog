use super::{Article, DbArticle, Summary};
use crate::{common::*, util::file};
use bson::{doc, oid::ObjectId, Document};
use log::info;
use mongodb::options::FindOptions;
use std::collections::BTreeMap;

// 保存文章，返回文章id
// 用户创建文章的时候，先调用这个接口，其中的title和content都先是默认值，然后将页面跳转到alter.html页面
pub fn save_article(article: Article) -> Result<String> {
    let mut article = article;
    article.time_update();
    let db_article = DbArticle::new(article);
    super::save(DbArticle::TABLE_NAME, db_article)
}

// base 获取某个db_article
pub fn get_dbarticle(id: &str) -> Result<DbArticle> {
    super::get(DbArticle::TABLE_NAME, id)
}

// base 列出指定文章
pub fn list_articles(
    filter: Option<Document>,
    find_options: FindOptions,
) -> Result<Vec<DbArticle>> {
    super::list(DbArticle::TABLE_NAME, filter, find_options)
}

// 更新id对应的文章
// return : 返回更改的个数
pub fn update_article(id: &str, article: Article) -> Result<i64> {
    info!("update article is :{:?}", article);
    let mut article = article;
    article.time_update();
    let db_article = DbArticle::new(article);
    let mut d = db_article.to_document().unwrap();
    d.remove("create_time");
    super::update(DbArticle::TABLE_NAME, id, d)
}

// 删除对应的文章
pub fn remove_article(id: &str) -> Result<i64> {
    super::remove(DbArticle::TABLE_NAME, id)
}

pub fn get_publish_article(id: &str) -> Result<Article> {
    get_dbarticle(id).map(|db_article| {
        db_article
            .into_publish()
            .expect("cant find this publish article")
    })
}

pub fn get_edit_article(id: &str) -> Result<Article> {
    get_dbarticle(id).map(|db_article| db_article.into_edit().expect("cant find this edit article"))
}

// 列出所有发布的文
#[allow(dead_code)]
pub fn list_publish_articles() -> Result<Vec<Article>> {
    let find_options = FindOptions::builder().build();
    let filter = Some(doc! {"status":DbArticle::PUBLISHED});
    list_articles(filter, find_options).map(|v| {
        v.into_iter()
            // filter_map是将option中的none的数据过滤，然后再unwrap()
            // 所以fn的返回值就是option
            .filter_map(|db_article| db_article.into_publish())
            .collect()
    })
}

// 列出所有可编辑的文章，一般来说就是列出所有文章
#[allow(dead_code)]
pub fn list_edit_articles() -> Result<Vec<Article>> {
    let find_options = FindOptions::builder().build();
    list_articles(Some(doc! {}), find_options).map(|v| {
        v.into_iter()
            .filter_map(|db_article| db_article.into_edit())
            .collect()
    })
}

// 列出已经发布的n篇文章
pub fn list_recent_articles(num: i64) -> Result<Vec<Article>> {
    let find_options = FindOptions::builder()
        .sort(Some(doc! {"last_update_time":-1}))
        .limit(num)
        .build();
    let filter = Some(doc! {"status":DbArticle::PUBLISHED, "last_publish_time": {"$ne":null}});
    let mut articles: Vec<Article> = list_articles(filter, find_options).map(|v| {
        v.into_iter()
            .filter_map(|db_article| db_article.into_publish())
            .collect()
    })?;
    articles.sort_by(|a, b| {
        a.update_time
            .as_ref()
            .unwrap()
            .cmp(&b.update_time.as_ref().unwrap())
    });
    Ok(articles)
}

pub fn list_summary_edit_articles() -> Result<Summary> {
    let find_options = FindOptions::builder()
        .projection(doc! { "publish": 0 })
        .allow_partial_results(true)
        .build();
    let filter = Some(doc! {});
    list_articles(filter, find_options)
        .map(|v| {
            v.into_iter()
                .filter_map(|db_article| db_article.into_edit())
                .map(|mut article| {
                    article.content = None;
                    article
                })
                .collect()
        })
        .map(summary)
}

pub fn list_summary_publish_articles() -> Result<Summary> {
    let find_options = FindOptions::builder()
        .projection(doc! { "edit": 0 })
        .allow_partial_results(true)
        .build();
    let filter = Some(doc! {"status":DbArticle::PUBLISHED});
    list_articles(filter, find_options)
        .map(|v| {
            v.into_iter()
                .filter_map(|db_article| db_article.into_publish())
                .map(|mut article| {
                    article.content = None;
                    article
                })
                .collect()
        })
        .map(summary)
}

// 根据catagory进行分类汇总
fn summary(articles: Vec<Article>) -> Summary {
    let mut m = BTreeMap::new();
    let no_catagory = "未分类".to_string();
    for article in articles.into_iter() {
        if let Some(c) = &article.catagory {
            if c.trim() != "" {
                (*m.entry(c.clone()).or_insert(vec![])).push(article);
                continue;
            }
        }
        (*m.entry(no_catagory.clone()).or_insert(vec![])).push(article);
    }
    m
}

// 将数据发布
pub fn publish_article(id: &str, article: Article) -> Result<i64> {
    info!("publish article is :{:?}", article);
    let mut article = article;
    article.time_update();
    let mut db_article = DbArticle::new(article.clone());
    db_article.last_publish_time = article.update_time.clone();
    db_article.publish = Some(article);
    db_article.status = Some(DbArticle::PUBLISHED);
    let mut d = db_article.to_document().unwrap();
    d.remove("create_time");
    let filter = doc! {"_id" => ObjectId::with_string(id)?};
    let update = doc! {"$set":d};
    Ok(table(DbArticle::TABLE_NAME)
        .update_one(filter, update, None)
        .map(|x| x.modified_count)?)
}

pub fn dump() {
    let dump_path = "md/";
    list_edit_articles()
        .expect("unwrap edit articles error")
        .iter()
        .for_each(|article| {
            if let Some(cat) = article.catagory.clone() {
                let dir = format!("{}{}", dump_path, cat);
                if let Err(_) = std::fs::create_dir_all(dir.clone()) {
                    return;
                }
                let file_path = format!("{}/{}.md", dir, article.title.clone().unwrap());
                file::save_to_file(file_path, article.content.clone().unwrap())
                    .expect("save failed");
            }
        });
}

mod test {

    #[test]
    fn test_save_article() {
        let article = super::Article {
            _id: None,
            title: Some("haa".to_string()),
            content: Some("heihei".to_string()),
            catagory: None,
            tag: None,
            update_time: None,
            status: None,
        };
        println!("{:?}", super::save_article(article));
    }

    #[test]
    fn test_get_publish_article() {
        println!(
            "{:?}",
            super::get_publish_article("5e78871300326e3600196a38")
        );
    }

    #[test]
    fn test_get_edit_article() {
        println!("{:?}", super::get_edit_article("5e78871300326e3600196a38"));
    }

    #[test]
    fn test_update_article() {
        let id = "5e78871300326e3600196a38";
        let article = super::Article {
            _id: None,
            title: Some("haa2".to_string()),
            content: Some("heihei2".to_string()),
            catagory: None,
            tag: None,
            update_time: None,
            status: None,
        };
        println!("{:?}", super::update_article(id, article));
    }

    #[test]
    fn test_publish_article() {
        let id = "5e78871300326e3600196a38";
        let article = super::Article {
            _id: None,
            title: Some("haa3".to_string()),
            content: Some("heihei3".to_string()),
            catagory: None,
            tag: None,
            update_time: None,
            status: None,
        };
        println!("{:?}", super::publish_article(id, article));
    }

    #[test]
    fn test_list_article() {
        println!("{:?}", super::list_edit_articles());
    }

    #[test]
    fn test_summary_article() {
        println!("{:?}", super::list_summary_edit_articles());
    }

    #[test]
    fn test_dump() {
        super::dump();
    }

    // 从文件中导入markdown到系统中
    #[test]
    fn test_parse_from_file() {
        if let Ok(paths) = std::fs::read_dir("/Users/qinxiaoguang01/gitpage/blog/org") {
            for path in paths {
                let path_name = path.unwrap().path();
                if let Ok(sec_paths) = std::fs::read_dir(path_name) {
                    for sec_path in sec_paths {
                        let file_path = sec_path.unwrap().path();
                        let file_name =
                            String::from(file_path.file_name().unwrap().to_str().unwrap());
                        let new_path = file_path
                            .strip_prefix("/Users/qinxiaoguang01/gitpage/blog/org/")
                            .unwrap();
                        let new_path_str = new_path.to_str().unwrap();
                        let all: Vec<&str> = new_path_str.split("/").collect();
                        let cat = all[0];
                        if file_name.ends_with("md") {
                            if let Ok(content) = std::fs::read_to_string(file_path.clone()) {
                                let title = file_name.trim_end_matches(".md");
                                //println!("title:{:?},content is:{:?}", title, content);
                                let article = super::Article {
                                    _id: None,
                                    title: Some(title.to_string()),
                                    content: Some(content),
                                    catagory: Some(cat.to_string()),
                                    tag: None,
                                    update_time: None,
                                    status: None,
                                };
                                println!("{:?}", super::save_article(article));
                            }
                        }
                    }
                }
            }
        }
    }
}
