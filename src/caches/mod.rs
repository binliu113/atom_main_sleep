pub mod udpcli_cache;

// use std::collections::HashMap;
// use rocket::futures::lock::Mutex;
//
// pub struct Cache {
//     pub data: HashMap<String, String>,
// }
//
// impl Cache {
//     pub fn new() -> Self {
//         Self { data: Default::default() }
//     }
// }
//
// lazy_static::lazy_static! {
//     pub static ref CACHE_MUTEX: Mutex<Cache> = Mutex::new(Cache::new());
//
//     pub static ref LIST_MUTEX: Mutex<Vec<i32>> = Mutex::new(vec![]);
// }