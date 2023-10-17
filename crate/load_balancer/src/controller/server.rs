use rocket::{get, post, routes, Route, serde::json::Json};
use crate::{
  controller::tool::{ApiError, to_json, TokenHeader},
  establish_connection, 
  model::{Server, NewServer},
  schema::server,
};
use entity::{res::ResServer, req::ReqNewServer};
use diesel::prelude::*;

pub fn server() -> Vec<Route> {
    routes![get_all_server, register_server]
}

#[get("/get")]
fn get_all_server(_token: TokenHeader) -> Result<String, ApiError> {
    use crate::schema::server::dsl::*;

    let connection = &mut establish_connection();
    let result = server
        .select(Server::as_select())
        .load(connection)
        .map_err(|err| ApiError::conflict(err.to_string()))?;

    Ok(to_json(
        &result
          .iter()
          .map(|e| {
            ResServer::from(e)
          })
          .collect::<Vec<ResServer>>())
      )
}

#[post("/", format = "application/json", data = "<body>")]
fn register_server(_token: TokenHeader, body: Json<ReqNewServer>) ->  Result<String, ApiError> {
    let model = NewServer::from(&*body);

    let connection = &mut establish_connection();
    let result = diesel::insert_into(server::table)
        .values(&model)
        .returning(Server::as_returning())
        .get_result(connection)
        .map_err(|err| ApiError::conflict(err.to_string()))?;

    Ok(to_json(&ResServer::from(result)))
}
