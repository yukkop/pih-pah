use serde::Serialize;
use uuid::Uuid;

use crate::model::User;

#[derive(Serialize)]
pub struct ResUser {
    pub id: Uuid,
    pub name: String,
    pub account_name: String,
}

impl From<User> for ResUser {
    fn from(model: User) -> Self {
        ResUser {
            id: model.id,
            name: model.name,
            account_name: model.account_name,
        }
    }
}
