mod controller;

use std::thread;
use std::thread::sleep;
use std::time::Duration;
use rocket;
use rocket::{routes, catch, catchers};
use rocket::fairing::AdHoc;
use controller::udpcli_controller::{start, stop, list, create};

#[catch(404)]
fn catch_404() -> String {
    "not fond 404".to_string()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .register("/", catchers![catch_404])
        .mount("/api", routes![start,stop,list,create])
        .attach(AdHoc::on_liftoff("Liftoff Printer", |_| Box::pin(async move {
            thread::spawn(|| loop { sleep(Duration::from_secs(1)) });
        })))
        .launch()
        .await?;
    Ok(())
}