use diesel::prelude::*;
use uuid::Uuid;

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

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user)]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub password_hash: String,
    pub password_salt: Vec<u8>,
    pub account_name: &'a str,
    pub language_id: i32,
}
