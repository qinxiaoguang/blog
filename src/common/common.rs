use crate::*;
use actix_web::{error, HttpResponse};
use bson::ordered::OrderedDocument;
use failure::Fail;
use iota::iota;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use std::result;

const OK: i32 = 0;

#[derive(Serialize, Deserialize)]
pub struct Resp<T>
where
    T: Serialize,
{
    code: i32,
    msg: Option<String>,
    data: Option<T>,
}

// 使用以下的error和result，即可使用?表达式，原因在于，任何类型的error都可以自动的转换为Box<error>
pub type Error = BizError;
pub type Result<T> = result::Result<T, Error>;
pub type CommonResp = Result<HttpResponse>;

// error code
iota! {
    const NORMAL_ERROR: i32 = 10000 << iota;
    , VALIDATE_ERROR
}

// 为什么用Fail,而不是实现 std::error::Error呢，因为actix_web::error::Error中实现了From<Fail> ，所以
// 可以将Fail直接转为actix_web::error::Error
#[derive(Fail, Debug, Serialize, Deserialize)]
pub enum BizError {
    #[fail(display = "Validation error on field: {}", field)]
    ValidateError { field: String },
    #[fail(display = "An internal error occured")]
    InternalError,

    #[fail(display = "An internal error occured: {}", field)]
    CommonError { field: String },
    #[fail(display = "login auth failed")]
    LoginAuthFailedError,
}

// 为了便于使用?号运算符，为所有的error类型实现From方法
impl<T> std::convert::From<T> for BizError
where
    T: std::error::Error,
{
    fn from(err: T) -> Self {
        BizError::CommonError {
            field: err.to_string(),
        }
    }
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            BizError::ValidateError { .. } => {
                let resp = Resp::err(VALIDATE_ERROR, &self.to_string());
                HttpResponse::BadRequest().json(resp)
            }
            _ => {
                let resp = Resp::err(NORMAL_ERROR, &self.to_string());
                HttpResponse::BadRequest().json(resp)
            }
        }
    }
}

impl<T: Serialize> Resp<T> {
    pub fn ok(data: T) -> Self {
        Resp {
            code: OK,
            msg: None,
            data: Some(data),
        }
    }

    pub fn to_json(&self) -> CommonResp {
        Ok(HttpResponse::Ok().json(self))
    }
}

impl Resp<()> {
    pub fn err(code: i32, msg: &str) -> Self {
        Resp {
            code: code,
            msg: Some(msg.to_owned()),
            data: None,
        }
    }

    pub fn errm(msg: &str) -> Self {
        Resp {
            code: NORMAL_ERROR,
            msg: Some(msg.to_owned()),
            data: None,
        }
    }
}

// 以下均是对mongo的统一封装
// mongo 统一处理
pub fn table(coll_name: &str) -> Collection {
    MONGO
        .database(&GLOBAL_CONF.mongo.db_name)
        .collection(coll_name)
}

// 为Cursor实现 to_vec
pub trait CursorToVec {
    fn to_vec<'a, T: Deserialize<'a>>(&mut self) -> Vec<T>;
}

impl CursorToVec for mongodb::Cursor {
    fn to_vec<'a, T: Deserialize<'a>>(&mut self) -> Vec<T> {
        self.map(|item| {
            let doc = item.unwrap();
            let bson = bson::Bson::Document(doc);
            bson::from_bson(bson).unwrap()
        })
        .collect()
    }
}

// 将一些struct转换为document
pub trait IntoDocument<'a>
where
    Self: Sized + Serialize + Deserialize<'a>,
{
    fn to_document(&self) -> Option<OrderedDocument> {
        let mid = bson::to_bson(self)
            .ok()
            .map(|x| x.as_document().unwrap().to_owned());

        mid.map(|mut doc| {
            let keys = doc.keys();
            let rm: Vec<String> = keys
                .filter(|k| doc.is_null(k))
                .map(|x| x.to_owned())
                .collect();
            for x in rm {
                doc.remove(&x);
            }
            doc
        })
    }
}
// mongo封装完毕
