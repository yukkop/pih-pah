use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReqNewUser<'a> {
  pub name: &'a str,
  pub password: &'a str,
  pub account_name: &'a str,
  pub language_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ReqLogin<'a> {
  pub account_name: &'a str,
  pub password: &'a str,
}
