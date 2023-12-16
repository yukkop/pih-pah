use crate::lobby::single::SingleLobbyPlugins;
use crate::map::MapState;
use crate::world::LinkId;
use bevy::app::{App, Plugin};
use bevy::ecs::event::Event;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Color, Component, Entity, Resource, States};
use bevy::reflect::Reflect;
use renet::transport::NETCODE_USER_DATA_BYTES;
use renet::ClientId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::client::ClientLobbyPlugins;
use super::host::HostLobbyPlugins;

pub const PROTOCOL_ID: u64 = 7;

/// An enumeration representing the states of a lobby system.
///
/// The [`LobbyState`] enum is used to define the various states that a lobby system can be in.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LobbyState {
    /// Indicates that the lobby system is in no specific state (default state).
    #[default]
    None = 0,
    /// Represents the state where a single player is present in the lobby.
    Single = 1,
    /// Represents the state where a player is hosting the lobby.
    Host = 2,
    /// Represents the state where a player is a client in the lobby.
    Client = 3,
}

/// Represents different types of messages that a server can send.
///
/// This enum is used to encapsulate various messages that a server
/// in a multiplayer game may need to send.
/// Each variant of the enum represents a different type of message
/// with its own associated data.
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    /// Sent when initializing a connection with a client.
    ///
    /// This message includes the client's ID and their initial map state.
    ///
    /// # Fields
    ///
    /// * `id` - Unique identifier for the connecting client.
    /// * `map_state` - Initial state of the client's map.
    InitConnection { id: ClientId, map_state: MapState },
    /// Sent to notify a change in the map's state.
    ///
    /// # Fields
    ///
    /// * `map_state` - The new state of the map.
    ChangeMap { map_state: MapState },
    /// Indicates that a player has connected to the server.
    ///
    /// # Fields
    ///
    /// * `id` - Unique identifier for the player.
    /// * `color` - The color assigned to the player.
    /// * `username` - The player's chosen username.
    PlayerConnected {
        id: PlayerId,
        color: Color,
        username: String,
    },
    /// Indicates that a player has disconnected from the server.
    ///
    /// # Fields
    ///
    /// * `id` - Unique identifier for the player who has disconnected.
    PlayerDisconnected { id: PlayerId },
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MapLoaderState {
    Yes,
    #[default]
    No,
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

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<PlayerId, PlayerData>,
    pub players_seq: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum PlayerId {
    HostOrSingle,
    Client(ClientId),
}

impl PlayerId {
    pub fn client_id(&self) -> Option<ClientId> {
        match self {
            PlayerId::HostOrSingle => None,
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

#[derive(Debug, Default, Serialize, Deserialize, Component, Resource, Reflect)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub sprint: bool,
    pub turn_horizontal: f32,
    pub turn_vertical: f32,
    pub special: bool,
}

#[derive(Debug, Component)]
pub struct Character {
    pub id: PlayerId,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct PlayerTransportData {
    pub position: Vec3,
    pub rotation: Quat,
    pub player_view: PlayerView,
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

#[derive(Debug, Component, Default, Serialize, Deserialize, Clone, Copy, Reflect)]
pub struct PlayerView{
    pub direction: Quat,
    pub distance: f32,
}

impl PlayerView {
    pub fn new(direction: Quat, distance: f32) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

#[derive(Debug, Event)]
pub struct ChangeMapLobbyEvent(pub MapState);

pub struct LobbyPlugins;

impl Plugin for LobbyPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ChangeMapLobbyEvent>()
            .add_state::<LobbyState>()
            .add_state::<MapLoaderState>()
            .init_resource::<HostResource>()
            .init_resource::<ClientResource>()
            .add_plugins((SingleLobbyPlugins, HostLobbyPlugins, ClientLobbyPlugins));
    }
}
