use crate::{
  controller::tool::{to_json, ApiError, TokenHeader},
  establish_connection,
  model::Country,
};
use diesel::prelude::*;
use entity::res::ResCountry;
use rocket::{get, routes, Route};

pub fn country() -> Vec<Route> {
  routes![get_all_countries]
}

#[get("/get")]
async fn get_all_countries(_token: TokenHeader) -> Result<String, ApiError> {
  use crate::schema::country::dsl::*;

  let connection = &mut establish_connection();
  let result = country
    .select(Country::as_select())
    .load(connection)
    .map_err(|err| ApiError::conflict(err.to_string()))?;

  Ok(to_json(
    &result
      .iter()
      .map(ResCountry::from)
      .collect::<Vec<ResCountry>>(),
  ))
}
