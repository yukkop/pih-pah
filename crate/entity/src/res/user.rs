use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ResUser {
  pub id: Uuid,
  pub name: String,
  pub account_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Me {
  pub name: String,
  pub account_name: String,
}
