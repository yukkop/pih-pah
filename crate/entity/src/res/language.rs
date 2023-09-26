use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResLanguage {
    pub id: i32,
    pub name: String,
}
