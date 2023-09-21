#[macro_use] extern crate rocket;
use load_balancer::controller;

#[get("/")]
fn index() -> &'static str {
    "Status ok!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/country", controller::coutry())
}
