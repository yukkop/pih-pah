use serde::Serialize;

use crate::model::Language;

#[derive(Serialize)]
pub struct ResLanguage {
    pub id: i32,
    pub name: String,
}

impl From<Language> for ResLanguage {
    fn from(model: Language) -> Self {
        ResLanguage {
            id: model.id,
            name: model.name,
        }
    }
}

impl From<&Language> for ResLanguage {
    fn from(model: &Language) -> Self {
        ResLanguage {
            id: model.id,
            name: model.name.clone(),
        }
    }
}
