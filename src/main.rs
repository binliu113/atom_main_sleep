mod controller;

use rocket;
use structopt::StructOpt;
use rocket::fairing::AdHoc;
use rocket::{routes, catch, catchers};
use controller::udpcli_controller::{start, stop, list, create};
use rocket_learning::caches::{CONFIGURATION, SKT_LIST};
use rocket_learning::sqlist_model::manage::{ArgsOpt, Event, ClearDB, ResetDB, TestDB};

#[catch(404)]
fn catch_404() -> String {
    "not fond 404".to_string()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    match ArgsOpt::from_args() {
        ArgsOpt::Manage { event } => {
            match event {
                Event::ClearDB => ClearDB::run().await,
                Event::ResetDB => ResetDB::run().await,
                Event::TestDB => TestDB::run().await
            }
        }
        ArgsOpt::Server { .. } => {
            let _rocket = rocket::build()
                .register("/", catchers![catch_404])
                .mount("/api", routes![start,stop,list,create])
                .attach(AdHoc::on_liftoff("Liftoff Printer", |_| Box::pin(async move {
                    CONFIGURATION.lock().await;
                    SKT_LIST.lock().await;
                })))
                .launch()
                .await?;
            Ok(())
        }
    }
}