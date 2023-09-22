use rocket::{State, Request};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::serde_json;
use serde::Serialize;

use super::ApiError;

pub fn to_json<T: Serialize>(entity: &T) -> String {
    serde_json::to_string(entity).expect("Failed to serialize user to JSON")
}

// pub fn is_authorized(state: &State<Status>) -> Result<(), ApiError> {
//   // if **state == Status::Unauthorized {
//   //   return Err(ApiError::conflict_str("Unauthorized"));
//   // }
//   if req.headers().contains("X-Unauthorized") {
//     return Err(Status::Unauthorized);
//   }
//   
//     Ok(())
// }

pub struct Token(String);

#[derive(Debug)]
pub enum ApiTokenError {
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
