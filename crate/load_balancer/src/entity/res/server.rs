use crate::model::Server;
use entity::res::ResServer;

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
