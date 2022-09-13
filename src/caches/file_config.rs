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
    pub tables: Option<Vec<Tables>>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ConfigFile {
    pub sqlite: Sqlite,
}

impl ConfigFile {
    pub fn new() -> Self {
        match ConfigFile::config_parser() {
            None => {
                Self {
                    sqlite: Default::default()
                }
            }
            Some(config) => {
                Self {
                    sqlite: config.sqlite.clone()
                }
            }
        }
    }

    fn config_parser() -> Option<ConfigFile> {
        let mut p = env::current_dir().unwrap();
        p.push("Config.toml");
        match toml::from_slice::<ConfigFile>(&fs::read(&p).unwrap()) {
            Ok(r) => { Some(r) }
            _ => None
        }
    }

    pub fn get_sqlite_path(&self) -> String {
        self.sqlite.pathname.clone()
    }
}