mod lib;

use rocket;
use rocket::{get, routes};
use rocket::serde::{Deserialize, json::Json};

#[get("/world")]
fn world() -> &'static str {
    "d"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/learning", routes![world])
        .launch()
        .await?;
    Ok(())
}
