use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReqNewServer<'a> {
  pub name: &'a str,
  pub address: &'a str,
  pub country_id: i32,
}
