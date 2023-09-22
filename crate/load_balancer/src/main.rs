#[macro_use] extern crate rocket;
use load_balancer::controller;

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
      .mount("/server", controller::server())
      .launch()
      .await?;

    Ok(())
}
