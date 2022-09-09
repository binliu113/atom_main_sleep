use structopt::StructOpt;
use super::super::caches::CONFIGURATION;

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
    pub async fn run() ->Result<(), rocket::Error> {
        let config = CONFIGURATION.lock().await;
        let statement = config.sqlist.conn.execute(
            "DELETE FROM users;",
        );
        match statement {
            Ok(_) => {
                println!("清除数据操作成功！！");
                Ok(())
            }
            Err(_) => {
                panic!("清除数据操作失败！！");
            }
        }
    }
}

pub struct ResetDB {}

impl ResetDB {
    pub async fn run() ->Result<(), rocket::Error> {
        Ok(())
    }
}

