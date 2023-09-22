use rocket::{post, Route, routes, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::dto::res::ResUser;
use crate::establish_connection;
use crate::model::{NewUser, User};
use crate::schema::user;
use diesel::prelude::*;
use crate::controller::tool::{ApiError, to_json};
use crate::dto::req::ReqNewUser;

// use super::tool::{api_error::ApiError, shared::to_json};

pub fn user() -> Vec<Route> {
    routes![register]
}

#[post("/register", format = "application/json", data = "<body>")]
fn register(body: Json<ReqNewUser>) ->  Result<String, ApiError> {
    // // Сохранение соли и хеша в "базе данных"
    // db_emulation.push((hash, salt));
    //
    // // Верификация пароля
    // let check_password = "new_password123"; // пароль для проверки
    // let (stored_hash, stored_salt) = &db_emulation[0];
    // let matches = argon2::verify_encoded(stored_hash, check_password.as_bytes()).unwrap();
    // println!("Password matches: {}", matches);

    let model = NewUser::from(&*body);

    let connection = &mut establish_connection();
    let result = diesel::insert_into(user::table)
        .values(&model)
        .returning(User::as_returning())
        .get_result(connection)
        .map_err(|err| ApiError::conflict(err.to_string()))?;

    Ok(to_json(&ResUser::from(result)))
}

