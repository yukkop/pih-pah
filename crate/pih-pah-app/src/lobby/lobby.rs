use crate::lobby::single::SingleLobbyPlugins;
use crate::world::LinkId;
use bevy::app::{App, Plugin};
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Color, Component, Entity, Resource, States};
use renet::transport::NETCODE_USER_DATA_BYTES;
use renet::ClientId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::client::ClientLobbyPlugins;
use super::host::HostLobbyPlugins;

pub const PROTOCOL_ID: u64 = 7;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LobbyState {
    #[default]
    None = 0,
    Single = 1,
    Host = 2,
    Client = 3,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    InitConnection {
        id: ClientId,
    },
    PlayerConnected {
        id: PlayerId,
        color: Color,
        username: String,
    },
    PlayerDisconnected {
        id: PlayerId,
    },
}

#[derive(Resource)]
pub struct Username(pub String);

impl Default for Username {
    fn default() -> Self {
        Self("noname".to_string())
    }
}

impl Username {
    pub fn to_netcode_data(
        &self,
    ) -> Result<[u8; NETCODE_USER_DATA_BYTES], Box<dyn std::error::Error>> {
        let mut data = [0u8; NETCODE_USER_DATA_BYTES];
        if self.0.len() > NETCODE_USER_DATA_BYTES - 8 {
            let err = Err(From::from("Your username to long"));
            log::error!("{:?}", err);
            return err;
        }
        data[0..8].copy_from_slice(&(self.0.len() as u64).to_le_bytes());
        data[8..self.0.len() + 8].copy_from_slice(self.0.as_bytes());

        Ok(data)
    }

    pub fn from_user_data(
        user_data: &[u8; NETCODE_USER_DATA_BYTES],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut buffer = [0u8; 8];
        buffer.copy_from_slice(&user_data[0..8]);
        let mut len = u64::from_le_bytes(buffer) as usize;
        len = len.min(NETCODE_USER_DATA_BYTES - 8);
        let data = user_data[8..len + 8].to_vec();
        let username = String::from_utf8(data)?;

        Ok(username)
    }
}

#[derive(Debug, Default, Resource)]
pub struct ClientResource {
    pub address: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Default, Resource)]
pub struct HostResource {
    pub address: Option<String>,
    pub username: Option<String>,
}

pub struct LobbyPlugins;

impl Plugin for LobbyPlugins {
    fn build(&self, app: &mut App) {
        app.add_state::<LobbyState>()
            .init_resource::<HostResource>()
            .init_resource::<ClientResource>()
            .add_plugins((SingleLobbyPlugins, HostLobbyPlugins, ClientLobbyPlugins));
    }
}

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<PlayerId, PlayerData>,
    pub players_seq: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum PlayerId {
    Host,
    Client(ClientId),
}

impl PlayerId {
    pub fn client_id(&self) -> Option<ClientId> {
        match self {
            PlayerId::Host => None,
            PlayerId::Client(id) => Some(*id),
        }
    }
}

#[derive(Debug)]
pub struct PlayerData {
    pub entity: Entity,
    pub color: Color,
    pub username: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub sprint: bool,
    pub turn_left: bool,
    pub turn_right: bool,
}

#[derive(Debug, Component)]
pub struct Character {
    pub id: PlayerId,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct PlayerTransportData {
    pub position: Vec3,
    pub rotation: Quat,
    pub tied_camera_rotation: Quat,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct ObjectTransportData {
    pub position: Vec3,
    pub rotation: Quat,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct TransportData {
    pub players: HashMap<PlayerId, PlayerTransportData>,
    pub objects: HashMap<LinkId, ObjectTransportData>,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct TransportDataResource {
    pub data: TransportData,
}

#[derive(Debug, Component, Default)]
pub struct PlayerViewDirection(pub Quat);
