use load_balancer::controller;
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  rocket::build()
    .mount("/country", controller::country())
    .mount("/user", controller::user())
    .mount("/language", controller::language())
    .mount("/server", controller::server())
    .launch()
    .await?;

  Ok(())
}