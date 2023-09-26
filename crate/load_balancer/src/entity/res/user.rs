use uuid::Uuid;
use entity::res::ResUser;
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
