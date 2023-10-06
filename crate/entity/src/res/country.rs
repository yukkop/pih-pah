use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResCountry {
    pub id: i32,
    pub name: String,
    pub iso2: Option<String>,
    pub iso3: Option<String>,
}
