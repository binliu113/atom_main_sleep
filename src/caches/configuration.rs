use super::file_config::ConfigFile;
use super::db_conn_config::DBConn;

pub struct Configuration {
    pub config_file: ConfigFile,
    pub sqlist: DBConn,
}

impl Configuration {
    pub fn new() -> Self {
        let config_file = ConfigFile::new();
        let db_conn = DBConn::new(
            config_file.sqlite.pathname.clone()
        );
        Self {
            config_file: config_file.clone(),
            sqlist: db_conn,
        }
    }
}