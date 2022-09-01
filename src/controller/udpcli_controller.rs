use rocket::{get, post};
use rocket::serde::json::Json;
use rocket_learning::udpcli_cache::SKT_LIST;
use rocket_learning::utils::resp_struct::{RespJson, udpcli_struct, RespCode};
use rocket_learning::utils::socket_util::UDPSktTools;

#[post("/start", data = "<_start_pids>")]
pub async fn start(_start_pids: Json<udpcli_struct::form::StartPids>) {}

#[post("/stop", data = "<_stop_pids>")]
pub async fn stop(_stop_pids: Json<udpcli_struct::form::StopPids>) {}

#[get("/list")]
pub async fn list() -> Json<RespJson<Vec<udpcli_struct::resp::ShowListData>>> {
    let mut resp_vec: Vec<udpcli_struct::resp::ShowListData> = Vec::new();
    for (key, skt_tools) in SKT_LIST.lock().await.iter() {
        let _ = &mut resp_vec.push(
            udpcli_struct::resp::ShowListData {
                hash_key: key.clone(),
                name: skt_tools.name.clone(),
                ip: skt_tools.ip.clone(),
                port: skt_tools.port.clone(),
            }
        );
    }
    RespJson::new(resp_vec).body(RespCode::GetSuc)
}

#[post("/create", data = "<create_params>")]
pub async fn create(create_params: Json<udpcli_struct::form::CreateParams>) -> Json<RespJson<bool>> {
    let args = create_params;
    let _key = format!("{}_{}", args.ip.clone(), args.port.clone());
    let skt = UDPSktTools {
        name: args.name.to_string(),
        ip: args.ip.to_string(),
        port: args.port as u16,
        online: false,
        skt: Err(true),
    };
    let hash_key = skt.gen_sha256(&_key as &str);
    let bol = match SKT_LIST.lock().await.get(&hash_key as &str) {
        None => Ok(()),
        Some(_) => Err(()),
    };
    match bol {
        Err(_) => RespJson::new(false).body(RespCode::CreateErr),
        Ok(_) => {
            SKT_LIST.lock().await.insert(hash_key, skt);
            RespJson::new(true).body(RespCode::CreateSuc)
        }
    }
}