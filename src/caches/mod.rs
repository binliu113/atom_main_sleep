pub mod configuration;
mod init_file_config;
mod sqlist_config;

use std::collections::HashMap;

use rocket::futures::lock::Mutex;

use super::utils::socket_util::UDPSktTools;

use configuration::{Configuration};

lazy_static::lazy_static! {

    pub static ref SKT_LIST: Mutex<HashMap<String,UDPSktTools>> = Default::default();

    pub static ref CONFIGURATION: Mutex<Configuration> = Mutex::new(Configuration::new());
}