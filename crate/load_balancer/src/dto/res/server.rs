use serde::Serialize;
use uuid::Uuid;

use crate::model::Server;

#[derive(Serialize)]
pub struct ResServer {
    pub name: String,
    pub address: String,
    pub online: bool,
}

impl From<Server> for ResServer {
    fn from(model: Server) -> Self {
        Self {
            name: model.name,
            address: model.address,
            online: model.online,
        }
    }
}

impl From<&Server> for ResServer {
    fn from(model: &Server) -> Self {
        Self {
            name: model.name.clone(),
            address: model.address.clone(),
            online: model.online,
        }
    }
}
