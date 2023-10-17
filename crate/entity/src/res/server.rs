use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResServer {
  pub name: String,
  pub address: String,
  pub online: bool,
}
