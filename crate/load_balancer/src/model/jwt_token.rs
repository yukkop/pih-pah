use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::jwt_token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct JwtToken {
    pub id: Uuid,
    pub token: String,
    pub active: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::jwt_token)]
pub struct NewJwtToken<'a> {
    pub id: Uuid,
    pub token: &'a str,
    pub active: bool,
}
