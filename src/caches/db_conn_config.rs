use sqlite::Connection;


pub struct DBConn {
    pub conn: Connection,
}

impl DBConn {
    pub fn new(sqlite_path: String) -> Self {
        match Connection::open(&sqlite_path) {
            Ok(_con) => {
                Self {
                    conn: _con
                }
            }
            Err(_) => {
                let p_info = format!("sqlist path: {:?} 数据库连接失败！", &sqlite_path);
                panic!("{}", &*p_info);
            }
        }
    }
}