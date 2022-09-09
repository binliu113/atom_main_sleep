use super::init_file_config::InitConfigFile;
use super::sqlist_config::SqlistConfig;

pub struct Configuration {
    pub config_file: InitConfigFile,
    pub sqlist: SqlistConfig,
}

impl Configuration {
    pub fn new() -> Self {
        let init_config_file = InitConfigFile::new();
        let sqlist_config = SqlistConfig::new(
            init_config_file.sqlite_pathname.clone()
        );
        Self {
            config_file: init_config_file,
            sqlist: sqlist_config,
        }
    }
}