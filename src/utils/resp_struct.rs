pub use rocket::serde::{Deserialize, Serialize, json::Json};


pub enum RespCode {
    Suc,
    GetSuc,
    ADDSuc,
    CreateSuc,
    UpdateSuc,
    DeleteSuc,
    Err,
    GetErr,
    AddErr,
    CreateErr,
    UpdateErr,
    DeleteErr,
}

/// # resp json
///
/// ```rust
///
/// use rocket::serde::json::Json;
/// use rocket_learning::utils::resp_struct::{Code, RespJson};
///
/// fn get() -> Json<RespJson<String>> {
///     Json(RespJson {
///         info: String::from("成功"),
///         code: 200,
///         data: String::from("ok")
///     })
/// }
/// ```
#[derive(Deserialize, Serialize)]
pub struct RespJson<T> {
    pub info: String,
    pub code: i32,
    pub data: T,
}

impl<T> RespJson<T> where T: Clone + Default {
    fn match_status(status: RespCode) -> Option<(i32, String)> {
        match status {
            RespCode::Suc => Some((200, "请求成功！".to_string())),
            RespCode::GetSuc => Some((200, "获取请求成功！".to_string())),
            RespCode::ADDSuc => Some((200, "添加请求成功！".to_string())),
            RespCode::CreateSuc => Some((200, "创建请求成功！".to_string())),
            RespCode::UpdateSuc => Some((200, "更新请求成功！".to_string())),
            RespCode::DeleteSuc => Some((200, "删除请求成功！".to_string())),
            RespCode::Err => Some((201, "请求失败！".to_string())),
            RespCode::GetErr => Some((202, "获取请求失败！".to_string())),
            RespCode::AddErr => Some((203, "添加请求失败！".to_string())),
            RespCode::UpdateErr => Some((204, "更新请求失败！".to_string())),
            RespCode::DeleteErr => Some((205, "删除请求失败！".to_string())),
            RespCode::CreateErr => Some((206, "创建请求失败！".to_string())),
        }
    }

    /// # New RespJson
    ///
    /// Example:
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use rocket::serde::json::Json;
    /// use rocket_learning::utils::resp_struct::{RespCode, RespJson};
    /// fn get()-> Json<RespJson<HashMap<&str, u32>>> {
    ///     let mut hash_map:HashMap<&str,u32> = HashMap::new();
    ///     hash_map.insert("k1",10);
    ///     hash_map.insert("k2",20);
    ///     RespJson::new(hash_map).body(RespCode::Suc)
    /// }
    /// ```
    pub fn new(data: T) -> Self {
        Self {
            info: Default::default(),
            code: Default::default(),
            data: data.clone(),
        }
    }

    /// # Done Json
    ///
    /// Example:
    ///
    /// ```
    /// use rocket::{get};
    /// use std::collections::HashMap;
    /// use rocket::serde::json::Json;
    /// use rocket_learning::utils::resp_struct::{RespCode, RespJson};
    /// #[get("/example")]
    /// fn get()-> Json<RespJson<HashMap<&str, u32>>> {
    ///     let mut hash_map:HashMap<&str,u32> = HashMap::new();
    ///     hash_map.insert("k1",10);
    ///     hash_map.insert("k2",20);
    ///     RespJson::new(hash_map).body(RespCode::Suc)
    /// }
    /// ```
    pub fn body(&self, status: RespCode) -> Json<RespJson<T>> {
        match Self::match_status(status) {
            None => {
                Json(RespJson {
                    info: self.info.clone(),
                    code: self.code.clone(),
                    data: self.data.clone(),
                })
            }
            Some((code, info)) => {
                Json(RespJson {
                    info: info.clone(),
                    code: code.clone(),
                    data: self.data.clone(),
                })
            }
        }
    }
}

pub mod delay_struct {
    pub mod form {
        use rocket::form::{FromForm};
        use rocket::serde::{Deserialize, Serialize};

        #[derive(FromForm)]
        pub struct ApiParams {
            pub secs: u64,
        }

        #[derive(Deserialize, Serialize, Clone)]
        pub struct Input {
            pub key: String,
            pub value: String,
        }

        #[derive(Deserialize, Serialize, Clone)]
        pub struct Values {
            pub item: i32,
        }
    }

    pub mod resp {
        use rocket::serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Clone)]
        pub struct Test {
            pub start_datetime: String,
            pub end_datetime: String,
        }
    }
}

pub mod udpcli_struct {
    pub mod form {
        use rocket::serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Clone)]
        pub struct StartPids {}

        #[derive(Deserialize, Serialize, Clone)]
        pub struct StopPids {}

        #[derive(Deserialize, Serialize, Clone)]
        pub struct CreateParams {
            pub name: String,
            pub skt_type: String,
            pub ip: String,
            pub port: u16,
        }
    }

    pub mod resp {
        use rocket::serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Clone)]
        pub struct ShowListData {
            pub name: String,
            pub ip: String,
            pub port: u16,
            pub hash_key: String,
        }
    }
}