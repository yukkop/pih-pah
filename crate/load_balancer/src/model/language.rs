use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::language)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Language {
  pub id: i32,
  pub name: String,
}
