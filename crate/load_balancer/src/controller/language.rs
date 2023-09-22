use rocket::{get, routes, Route, State, http::Status, Request};
use crate::{
  controller::tool::{ApiError, to_json, TokenHeader},
  establish_connection, 
  dto::res::ResLanguage,
  model::Language,
};
use diesel::prelude::*;

pub fn language() -> Vec<Route> {
    routes![get_all]
}

#[get("/get")]
async fn get_all(_token: TokenHeader) -> Result<String, ApiError> {
    use crate::schema::language::dsl::*;

    let connection = &mut establish_connection();
    let result = language
        .select(Language::as_select())
        .load(connection)
        .map_err(|err| ApiError::conflict(err.to_string()))?;

    Ok(to_json(
        &result
          .iter()
          .map(|e| {
            ResLanguage::from(e)
          })
          .collect::<Vec<ResLanguage>>())
      )
}

