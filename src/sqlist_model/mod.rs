pub mod manage;
pub mod skt_list_model;
pub mod util;
pub mod wrapper;


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
                let p_info = format!(
                    "sqlist path: {:?} 数据库连接失败！",
                    &sqlite_path
                );
                panic!("{}", &*p_info);
            }
        }
    }

    pub fn execute_query(&self, query: String) -> bool {
        let debug_query = SQLITE_CONF.debug_query.unwrap();
        match self.conn.execute(query.clone()) {
            Ok(_) => {
                if debug_query {
                    log::info!(
                        "[36m{:^6}[0m",
                        query.clone()
                    );
                }
                true
            }
            Err(e) => {
                log::error!(
                    "[31m{:^6} | Err: “{:^6}” |[0m",
                    query.clone(),
                    e.message.unwrap()
                );
                false
            }
        }
    }
}

lazy_static::lazy_static! {
    pub static ref SQLITE_MODEL: Mutex<DBConn> = Mutex::new(DBConn::new());

    pub static ref SQLITE_CONF: Sqlite  = CONFIGURATION.config.sqlite.clone();
}