use rocket::{post, Route, routes, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::controller::tool::api_error::ApiError;
use crate::establish_connection;
use crate::model::{NewUser, User};
use crate::schema::user;
use diesel::prelude::*;

use super::tool::shared::to_json;

// use super::tool::{api_error::ApiError, shared::to_json};

pub fn user() -> Vec<Route> {
    routes![register]
}

use argon2::{Config, ThreadMode, Variant, Version};
use rand::Rng;

fn get_argon2_config() -> Config<'static> {
    Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 4096,
        time_cost: 192,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32,
    }
}

#[post("/register", format = "application/json", data = "<body>")]
fn register(body: Json<NewUser>) ->  Result<String, ApiError> {
    let new_user = body;

    let connection = &mut establish_connection();
    let result = diesel::insert_into(user::table)
        .values(&*new_user)
        .returning(User::as_returning())
        .get_result(connection)
        .map_err(|err| ApiError::conflict(err.to_string()))?;

    Ok(to_json(&result))
}

