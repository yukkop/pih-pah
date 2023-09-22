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

struct Token(String);

#[derive(Debug)]
enum ApiTokenError {
    Missing,
    Invalid,
}

impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;

    #[doc = " Derives an instance of `Self` from the incoming request metadata."]
    #[doc = ""]
    #[doc = " If the derivation is successful, an outcome of `Success` is returned. If"]
    #[doc = " the derivation fails in an unrecoverable fashion, `Failure` is returned."]
    #[doc = " `Forward` is returned to indicate that the request should be forwarded"]
    #[doc = " to other matching routes, if any."]
    #[must_use]
    #[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn from_request<'life0,'async_trait>(request: &'r Request<'life0>) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Outcome<Self,Self::Error> > + ::core::marker::Send+'async_trait> >
      where 
        'r:'async_trait,
        'life0:'async_trait,Self:'async_trait
    {
      let token = request.headers().get_one("token");

      match token {
          Some(token) => {
              // check validity
              Box::pin(async {
                Outcome::Success(Token(token.to_string()))
              })
          }
          None => 
              Box::pin(async {
                Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing))
              }),
      }
    }

    // fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
    //     let token = request.headers().get_one("token");
    //     match token {
    //         Some(token) => {
    //             // check validity
    //             request::Outcome::Success(Token(token.to_string()))
    //         }
    //         None => request::Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
    //     }
    // }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
      .mount("/country", controller::coutry())
      .mount("/user", controller::user())
      .mount("/language", controller::language())
}
