use rocket::{get, routes, Route};
use crate::{
  controller::tool::{ApiError, to_json},
  establish_connection, 
  dto::res::ResLanguage,
  model::Language,
};
use diesel::prelude::*;

pub fn language() -> Vec<Route> {
    routes![get_all]
}

#[get("/get")]
fn get_all() -> Result<String, ApiError> {
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

