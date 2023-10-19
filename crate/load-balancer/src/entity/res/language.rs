use crate::model::Language;
use entity::res::ResLanguage;

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
