use rocket::{post, Route, routes, serde::json::Json};
use uuid::Uuid;

use crate::dto::res::ResUser;
use crate::establish_connection;
use crate::model::{NewUser, User, JwtToken, NewJwtToken};
use crate::schema::user;
use crate::schema::jwt_token;
use diesel::prelude::*;
use crate::controller::tool::{ApiError, to_json, generate_token};
use crate::dto::req::{ReqNewUser, ReqLogin};

// use super::tool::{api_error::ApiError, shared::to_json};

pub fn user() -> Vec<Route> {
    routes![register, login]
}

#[post("/register", format = "application/json", data = "<body>")]
async fn register(body: Json<ReqNewUser<'_>>) ->  Result<String, ApiError> {
    let model = NewUser::from(&*body);

    let connection = &mut establish_connection();
    let result = diesel::insert_into(user::table)
        .values(&model)
        .returning(User::as_returning())
        .get_result(connection)
        .map_err(|err| ApiError::conflict(err.to_string()))?;

    Ok(to_json(&ResUser::from(result)))
}

#[post("/login", format = "application/json", data = "<body>")]
fn login(body: Json<ReqLogin>) -> Result<String, ApiError> {
    let result;
    {
      use crate::schema::user::dsl::*;

      let connection = &mut establish_connection();
      result = user
          .filter(account_name.eq(&*body.account_name))
          .select(User::as_select())
          .first(connection)
          .map_err(|_| ApiError::conflict_str("Password or account name not correct"))?;

      let matches: bool = argon2::verify_encoded(&result.password_hash, body.password.as_bytes()).unwrap();
      if !matches {
        return Err(ApiError::conflict_str("Password or account name not correct"));
      }
    }

    let generated_token = generate_token(result.id)
      .map_err(|err| ApiError::conflict(err.to_string()))?;

    let model = NewJwtToken {
      id: Uuid::new_v4(),
      token: &generated_token,
      active: true,
    };

    let connection = &mut establish_connection();
    let jwt_token = diesel::insert_into(jwt_token::table)
      .values(&model)
      .returning(JwtToken::as_returning())
      .get_result(connection)
      .map_err(|err| ApiError::conflict(err.to_string()))?;

    Ok(jwt_token.token)
}
