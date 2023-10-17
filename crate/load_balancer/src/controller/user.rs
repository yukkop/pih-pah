use crate::{
  controller::tool::{generate_token, to_json, ApiError},
  establish_connection,
  model::{JwtToken, NewJwtToken, NewUser, User},
  schema::{jwt_token, user},
};
use diesel::prelude::*;
use entity::{
  req::{ReqLogin, ReqNewUser},
  res::{Me, ResJwtToken},
};
use rocket::{get, post, routes, serde::json::Json, Route};
use uuid::Uuid;

use super::tool::TokenHeader;

pub fn user() -> Vec<Route> {
  routes![register, login, me]
}

#[post("/register", format = "application/json", data = "<body>")]
async fn register(body: Json<ReqNewUser<'_>>) -> Result<String, ApiError> {
  let model = NewUser::from(&*body);

  let connection = &mut establish_connection();
  let result = diesel::insert_into(user::table)
    .values(&model)
    .returning(User::as_returning())
    .get_result(connection)
    .map_err(|err| ApiError::conflict(err.to_string()))?;

  Ok(to_json(&Me::from(result)))
}

#[get("/me")]
fn me(token: TokenHeader) -> Result<String, ApiError> {
  use crate::schema::user::dsl::*;

  let connection = &mut establish_connection();
  let result = user
    .filter(id.eq(token.id))
    .select(User::as_select())
    .first(connection)
    .map_err(|err| ApiError::conflict(err.to_string()))?;

  Ok(to_json(&Me::from(result)))
}

#[post("/login", format = "application/json", data = "<body>")]
fn login(body: Json<ReqLogin>) -> Result<String, ApiError> {
  let result;
  let connection = &mut establish_connection();
  {
    use crate::schema::user::dsl::*;

    result = user
      .filter(account_name.eq(body.account_name))
      .select(User::as_select())
      .first(connection)
      .map_err(|_| ApiError::conflict_str("Password or account name not correct"))?;

    let matches: bool =
      argon2::verify_encoded(&result.password_hash, body.password.as_bytes()).unwrap();
    if !matches {
      return Err(ApiError::conflict_str(
        "Password or account name not correct",
      ));
    }
  }

  let generated_token =
    generate_token(result.id).map_err(|err| ApiError::conflict(err.to_string()))?;

  // TODO cringe
  let res = {
    use crate::schema::jwt_token::dsl::*;
    jwt_token
      .filter(token.eq(&generated_token))
      .select(JwtToken::as_select())
      .first(connection)
    // .map_err(|_| ApiError::conflict_str("Password or account name not correct"))?
  };
  if let Ok(jwt_token) = res {
    return Ok(to_json(&ResJwtToken::from(jwt_token)));
  }

  let model = NewJwtToken {
    id: Uuid::new_v4(),
    token: &generated_token,
    active: true,
  };

  let jwt_token = diesel::insert_into(jwt_token::table)
    .values(&model)
    .returning(JwtToken::as_returning())
    .get_result(connection)
    .map_err(|err| ApiError::conflict(err.to_string()))?;

  Ok(to_json(&ResJwtToken::from(jwt_token)))
}
