use rocket::{get, routes, Route};
use crate::{
  controller::tool::{ApiError, to_json},
  establish_connection, 
  dto::res::ResCountry,
  model::Country,
};
use diesel::prelude::*;

pub fn coutry() -> Vec<Route> {
    routes![get_all]
}

#[get("/get")]
async fn get_all() -> Result<String, ApiError> {
    use crate::schema::country::dsl::*;

    let connection = &mut establish_connection();
    let result = country
        .select(Country::as_select())
        .load(connection)
        .map_err(|err| ApiError::conflict(err.to_string()))?;

    Ok(to_json(
        &result
          .iter()
          .map(|e| {
            ResCountry::from(e)
          })
          .collect::<Vec<ResCountry>>())
      )
}

