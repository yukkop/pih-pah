use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::country)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub iso2: Option<String>,
    pub iso3: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub password_hash: String,
    pub password_salt: Vec<u8>,
    pub account_name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::language)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Language {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password_hash: String,
    pub password_salt: Vec<u8>,
    pub account_name: String,
}
