mod controller;
mod on_liftoff;

use std::env;
use rocket;
use log4rs;
use structopt::StructOpt;
use on_liftoff::OnLiftoff;
use rocket::fairing::AdHoc;
use rocket::tokio::sync::{mpsc};
use rocket::{routes, catch, catchers};
use rocket_atom::caches::{SKT_LIST, TX_SQL_CHANNEL, CONFIGURATION};
use controller::udpcli_controller::{start, stop, list, create};
use rocket_atom::sqlist_model::manage::{ArgsOpt, Event, ClearDB, ResetDB};


#[catch(404)]
fn catch_404() -> String {
    "not fond 404".to_string()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut cur_path = env::current_dir().unwrap();

    cur_path.push(CONFIGURATION.config.log4rs.config_path.as_str());

    log4rs::init_file(cur_path.as_path(), Default::default()).unwrap();

    match ArgsOpt::from_args() {
        ArgsOpt::Manage { event } => {
            match event {
                Event::ClearDB => ClearDB::run().await,
                Event::ResetDB => ResetDB::run().await,
            }
        }
        ArgsOpt::Server { .. } => {
            let _rocket = rocket::build()
                .register("/", catchers![catch_404])
                .mount("/api", routes![start,stop,list,create])
                .attach(AdHoc::on_liftoff("Liftoff Printer", |_| Box::pin(async move {
                    let (tx, rx) = mpsc::channel(2048);

                    *TX_SQL_CHANNEL.lock().await = Some(tx);

                    OnLiftoff::sql_event(rx).await;

                    SKT_LIST.lock().await;
                })))
                .launch()
                .await?;
            Ok(())
        }
    }
}