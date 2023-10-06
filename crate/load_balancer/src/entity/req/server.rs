use crate::model::NewServer;
use uuid::Uuid;
use entity::req::ReqNewServer;

impl<'a> From<ReqNewServer<'a>> for NewServer<'a> {
    fn from(req: ReqNewServer<'a>) -> Self {
        Self {
            id: Uuid::new_v4(), 
            name: req.name,
            online: false, // it will online when connect to reciever
            address: req.address,
            country_id: req.country_id,
        }
    }
}

impl<'a> From<&ReqNewServer<'a>> for NewServer<'a> {
    fn from(req: &ReqNewServer<'a>) -> Self {
        Self {
            id: Uuid::new_v4(), 
            name: req.name,
            online: false, // it will online when connect to reciever
            address: req.address,
            country_id: req.country_id,
        }
    }
}
