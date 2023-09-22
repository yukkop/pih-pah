use rocket::post;
use rocket::routes;
use rocket::Route;
use crate::controller::tool::{ApiError, to_json};
use crate::establish_connection;
use crate::model::Country;
use diesel::prelude::*;
use crate::dto::res::ResCountry;

pub fn coutry() -> Vec<Route> {
    routes![get_all]
}

#[post("/get")]
fn get_all() -> Result<String, ApiError> {
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

