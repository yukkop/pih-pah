use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::country)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub iso2: Option<String>,
    pub iso3: Option<String>,
}
