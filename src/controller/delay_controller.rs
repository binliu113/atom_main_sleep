use std::collections::HashMap;
use rocket::{get, post};
use rocket::tokio::time::{sleep, Duration};
use rocket_learning::utils::resp_struct::{RespJson, Json, delay_struct};
use chrono::prelude::{Local};
use rocket_learning::caches::{CACHE_MUTEX, LIST_MUTEX};


#[get("/test-get?<api_params..>")]
pub async fn delay(api_params: delay_struct::form::ApiParams) -> Json<RespJson<delay_struct::resp::Test>> {
    let start = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    sleep(Duration::from_secs(api_params.secs)).await;

    let data = delay_struct::resp::Test {
        start_datetime: start,
        end_datetime: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    RespJson::new("suc".to_string(), 200, data).body()
}


#[post("/test-post", data = "<input>")]
pub async fn test_post(input: Json<delay_struct::form::Input>) -> Json<RespJson<HashMap<String, String>>> {
    CACHE_MUTEX.lock().await.data.insert(input.key.clone(), input.value.clone());

    let data = CACHE_MUTEX.lock().await.data.clone();

    RespJson::new("suc".to_string(), 200, data).body()
}

#[post("/test-post-push", data = "<values>")]
pub async fn test_post_push(values: Json<delay_struct::form::Values>) -> Json<RespJson<String>> {
    LIST_MUTEX.lock().await.push(values.item.clone());

    RespJson::new("suc".to_string(), 200, "suc".to_string()).body()
}

#[get("/get-push-data")]
pub async fn get_push_data() -> Json<RespJson<Vec<i32>>> {
    let data = LIST_MUTEX.lock().await.clone();

    RespJson::new("suc".to_string(), 200, data).body()
}