use serde::Serialize;

use crate::model::Country;

#[derive(Serialize)]
pub struct ResCountry {
    pub id: i32,
    pub name: String,
    pub iso2: Option<String>,
    pub iso3: Option<String>,
}

impl From<Country> for ResCountry {
    fn from(model: Country) -> Self {
        ResCountry {
            id: model.id,
            name: model.name,
            iso2: model.iso2,
            iso3: model.iso3,
        }
    }
}

impl From<&Country> for ResCountry {
    fn from(model: &Country) -> Self {
        ResCountry {
            id: model.id,
            name: model.name.clone(),
            iso2: model.iso2.clone(),
            iso3: model.iso3.clone(),
        }
    }
}
