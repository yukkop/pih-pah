#[macro_use] extern crate rocket;
use load_balancer::controller;
use rocket::http::Status;
use rocket::request::{FromRequest, self, Outcome};
use rocket::{Request, Data};
use rocket::fairing::{Fairing, Info, Kind};

#[get("/")]
fn index() -> &'static str {
    "Status ok!"
}

// #[launch]
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
      .mount("/country", controller::coutry())
      .mount("/user", controller::user())
      .mount("/language", controller::language())
      .launch()
      .await?;

    Ok(())
}
