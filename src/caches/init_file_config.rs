use toml;
use std::fs;
use std::env;
use rocket::serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct InitConfigFile {
    pub sqlite_pathname: String,
}

impl InitConfigFile {
    pub fn new() -> Self {
        match InitConfigFile::config_parser() {
            None => {
                Self {
                    sqlite_pathname: Default::default(),
                }
            }
            Some(config) => {
                let mut cur_path = env::current_dir().unwrap();
                cur_path.push(config.sqlite_pathname.clone().as_str());
                Self {
                    sqlite_pathname: cur_path.clone().to_str().unwrap().to_string(),
                }
            }
        }
    }

    fn config_parser() -> Option<InitConfigFile> {
        let mut p = env::current_dir().unwrap();
        p.push("Config.toml");
        match toml::from_slice::<InitConfigFile>(&fs::read(&p).unwrap()) {
            Ok(r) => { Some(r) }
            _ => None
        }
    }

    pub fn get_sqlite_path(&self) -> String {
        self.sqlite_pathname.clone()
    }
}