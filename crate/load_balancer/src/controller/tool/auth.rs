use rocket::Request;
use rocket::http::Status;
use rocket::request::{Outcome, FromRequest};
use serde::{Serialize, Deserialize};

pub struct TokenHeader {
  pub id: Uuid,
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

impl<'r> FromRequest<'r> for TokenHeader {
    type Error = ApiTokenError;

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
            if let Ok(claims) = verify_token(token) {
              Box::pin(async move {
                Outcome::Success( TokenHeader { id: claims.sub.clone() })
              })
            } else {
              Box::pin(async {
                Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid))
              })
            }
          }
          None => 
            Box::pin(async {
              Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing))
            }),
      }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: Uuid,
    exp: usize,
}

use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use jsonwebtoken::errors::Error as JWTError;
use uuid::Uuid;

pub fn generate_token(id: Uuid) -> Result<String, JWTError> {
    let claims = Claims { 
        sub: id.to_owned(),
        exp: 10000000000, // TODO end time
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))?;
    Ok(token)
}

pub fn verify_token(token: &str) -> Result<Claims, JWTError> {
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?;
    Ok(token_data.claims)
}
