#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Status ok!"
}

#[post("/user/register")]
fn register() -> &'static str {
    "Status ok!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
