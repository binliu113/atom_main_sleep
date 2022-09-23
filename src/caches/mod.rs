pub mod configuration;
pub mod config_parser;

use std::collections::HashMap;

use rocket::tokio::sync::{Mutex, mpsc::{Sender}};

use configuration::{Configuration};

use super::utils::socket_util::UDPSktTools;

lazy_static::lazy_static! {
    pub static ref SKT_LIST: Mutex<HashMap<String,UDPSktTools>> = Default::default();

    pub static ref CONFIGURATION: Configuration = Configuration::new();

    pub static ref TX_SQL_CHANNEL: Mutex<Option<Sender<String>>> = Mutex::new(None);
}