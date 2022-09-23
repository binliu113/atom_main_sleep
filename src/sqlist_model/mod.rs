pub mod manage;
pub mod skt_list_model;
pub mod util;

use crate::sqlist_model::util::Model;

use skt_list_model::SktListModel;
use rocket::tokio::sync::{Mutex};
use sqlite::Connection;
use crate::caches::config_parser::Sqlite;
use super::caches::CONFIGURATION;


pub struct DBConn {
    pub conn: Connection,
}

impl DBConn {
    pub fn new() -> Self {
        let sqlite_path = CONFIGURATION.config.sqlite.pathname.clone();

        match Connection::open(&sqlite_path) {
            Ok(_con) => Self { conn: _con },
            Err(_) => {
                let p_info = format!("sqlist path: {:?} 数据库连接失败！", &sqlite_path);
                panic!("{}", &*p_info);
            }
        }
    }
}

lazy_static::lazy_static! {
    pub static ref MODEL_SKT_LIST: Mutex<SktListModel> = Mutex::new(SktListModel::new());
    pub static ref SQLITE_MODEL: Mutex<DBConn> = Mutex::new(DBConn::new());
    pub static ref SQLITE_CONF: Sqlite  = CONFIGURATION.config.sqlite.clone();
}