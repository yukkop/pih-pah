use serde::Deserialize;

use crate::model::NewServer;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ReqNewServer<'a> {
    pub name: &'a str,
    pub address: &'a str, 
    pub country_id: i32, 
}

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
