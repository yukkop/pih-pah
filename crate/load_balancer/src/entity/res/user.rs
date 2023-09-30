use entity::res::{ResUser, Me};
use crate::model::User;

impl From<User> for ResUser {
    fn from(model: User) -> Self {
        Self {
            id: model.id,
            name: model.name,
            account_name: model.account_name,
        }
    }
}

impl From<User> for Me {
    fn from(model: User) -> Self {
        Self {
            name: model.name,
            account_name: model.account_name,
        }
    }
}
