use toml;
use std::fs;
use std::env;
use rocket::serde::{Deserialize};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Tables {
    pub info: String,
    pub tablename: String,
    pub columns: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Sqlite {
    pub pathname: String,
    pub debug_query: Option<bool>,
    pub tables: Option<Vec<Tables>>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Log4rs {
    pub config_path: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ConfigParser {
    pub sqlite: Sqlite,
    pub log4rs: Log4rs,
}

impl ConfigParser {
    pub fn new() -> Self {
        match ConfigParser::config_parser() {
            None => Self {
                sqlite: Default::default(),
                log4rs: Default::default(),
            },
            Some(config) => Self {
                sqlite: config.sqlite.clone(),
                log4rs: config.log4rs.clone(),
            }
        }
    }

    fn config_parser() -> Option<ConfigParser> {
        let mut p = env::current_dir().unwrap();
        p.push("Config.toml");
        match toml::from_slice::<ConfigParser>(&fs::read(&p).unwrap()) {
            Ok(r) => { Some(r) }
            _ => None
        }
    }

    pub fn get_sqlite_path(&self) -> String {
        self.sqlite.pathname.clone()
    }
}