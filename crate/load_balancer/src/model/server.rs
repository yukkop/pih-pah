use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::server)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub online: bool,
    pub country_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::server)]
pub struct NewServer<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub address: &'a str,
    pub online: bool,
    pub country_id: i32,
}
