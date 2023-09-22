#[macro_use] extern crate rocket;
use load_balancer::controller;
use rocket::fairing::Fairing;

#[get("/")]
fn index() -> &'static str {
    "Status ok!"
}

// struct AuthFairing;
//
// #[rocket::async_trait]
// impl Fairing for AuthFairing {
//     fn info(&self) -> Info {
//         Info {
//             name: "AuthFairing",
//             kind: Kind::Request,
//         }
//     }
//
//     async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
//         let mut unauthorized = false;
//
//         if let Some(auth_header) = req.headers().get_one("Authorization") {
//             // Check auth_header
//             unauthorized = auth_header != "some_valid_token";
//         } else {
//             unauthorized = true;
//         }
//
//         if unauthorized {
//             // Mark the request as unauthorized using a custom header or shared state
//             req.local_cache(|| Status::Unauthorized);
//         }
//     }
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
      // .attach(AuthFairing)
      .mount("/country", controller::coutry())
      .mount("/user", controller::user())
      .mount("/language", controller::language())
}
