use structopt::StructOpt;
use super::super::caches::CONFIGURATION;
use super::SQLITE_MODEL;

#[derive(Debug, StructOpt)]
pub enum Event {
    /// 清除数据库数据
    #[structopt(name = "clear")]
    ClearDB,

    /// 重新设置数据库数据
    #[structopt(name = "reset")]
    ResetDB,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "client", about = "An example of StructOpt usage.", version = "0.1")]
pub enum ArgsOpt {
    /// 应用管理
    #[structopt(name = "manage")]
    Manage {
        /// 管理触发事件
        #[structopt(subcommand)]
        event: Event,
    },

    ///启动 webserver
    #[structopt(name = "server")]
    Server,
}

pub struct ClearDB {}

impl ClearDB {
    pub async fn run() -> Result<(), rocket::Error> {
        let config = CONFIGURATION.clone();

        let tables = &config.config.sqlite.tables.as_ref().unwrap().clone();

        let mut statement = String::from("");

        for (_index, table) in tables.iter().enumerate() {
            let t = format!("DROP TABLE {};", &table.tablename);
            statement += &*t;
        }

        let statement = SQLITE_MODEL.lock().await.conn.execute(statement);

        match statement {
            Ok(_) => {
                Ok(())
            }
            Err(_) => {
                panic!("操作失败！！");
            }
        }
    }
}

pub struct ResetDB {}

impl ResetDB {
    pub async fn run() -> Result<(), rocket::Error> {
        let config = CONFIGURATION.clone();
        let sqlite_model = SQLITE_MODEL.lock().await;

        let tables = &config.config.sqlite.tables.as_ref().unwrap().clone();

        let mut statement = String::from("");

        let mut d_statement = String::from("");

        for (_index, table) in tables.iter().enumerate() {
            let t = format!("CREATE TABLE {} ({});", table.tablename.clone(), table.columns.join(","));

            let d = format!("DROP TABLE {};", table.tablename.clone());

            d_statement += &*d;

            statement += &*t;
        }

        let statement_res = sqlite_model.conn.execute(&statement);

        match statement_res {
            Ok(_) => {
                Ok(())
            }
            Err(_) => {
                let _ = sqlite_model.conn.execute(&d_statement);
                let _ = sqlite_model.conn.execute(&statement);
                Ok(())
            }
        }
    }
}