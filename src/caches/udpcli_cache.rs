use rocket::futures::lock::Mutex;
use super::super::utils::socket_util::UDPSktTools;
use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref SKT_LIST: Mutex<HashMap<String,UDPSktTools>> = Default::default();
}