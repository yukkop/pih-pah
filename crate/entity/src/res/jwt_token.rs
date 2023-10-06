use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResJwtToken {
    pub token: String,
}
