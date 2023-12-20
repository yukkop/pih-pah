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
    InitConnection {
        id: ClientId,
        map_state: MapState,
    },
    /// Sent to notify a change in the map's state.
    ///
    /// # Fields
    ///
    /// * `map_state` - The new state of the map.
    ChangeMap {
        map_state: MapState,
    },
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
    PlayerDisconnected {
        id: PlayerId,
    },
    ProjectileSpawn {
        id: LinkId,
        color: Color,
    },
    ActorDespawn {
        id: LinkId,
    },
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

// TODO resource????????
#[derive(Debug, Default, Component, Reflect)]
pub struct PlayerInputs {
    input: Inputs,
    previouse_input: Inputs,
}

pub enum InputValue {
    Boolean(bool),
    Float(f32),
}

pub const THRESHOLD: f32 = 0.1;

impl PlayerInputs {
    pub fn insert_inputs(&mut self, input: Inputs) {
        self.previouse_input = self.input;
        self.input = input;
    }

    pub fn add(&mut self, input: Inputs) {
        self.input.up |= input.up;
        self.input.down |= input.down;
        self.input.left |= input.left;
        self.input.right |= input.right;
        self.input.jump |= input.jump;
        self.input.sprint |= input.sprint;
        self.input.turn_horizontal += input.turn_horizontal;
        self.input.turn_vertical += input.turn_vertical;
        self.input.special |= input.special;
        self.input.fire |= input.fire;
    }

    pub fn get(&self) -> Inputs {
        self.input
    }

    pub fn is_input_changed(&self, input_type: InputType) -> bool {
        match input_type {
            InputType::Up => self.input.up != self.previouse_input.up,
            InputType::Down => self.input.down != self.previouse_input.down,
            InputType::Left => self.input.left != self.previouse_input.left,
            InputType::Right => self.input.right != self.previouse_input.right,
            InputType::Jump => self.input.jump != self.previouse_input.jump,
            InputType::Sprint => self.input.sprint != self.previouse_input.sprint,
            InputType::TurnHorizontal => {
                self.input.turn_horizontal != self.previouse_input.turn_horizontal
            }
            InputType::TurnVertical => {
                self.input.turn_vertical != self.previouse_input.turn_vertical
            }
            InputType::Special => self.input.special != self.previouse_input.special,
            InputType::Fire => self.input.fire != self.previouse_input.fire,
        }
    }

    pub fn is_input_changed_to_true(&self, input_type: InputType) -> bool {
        match input_type {
            InputType::Up => !self.previouse_input.up && self.input.up,
            InputType::Down => !self.previouse_input.down && self.input.down,
            InputType::Left => !self.previouse_input.left && self.input.left,
            InputType::Right => !self.previouse_input.right && self.input.right,
            InputType::Jump => !self.previouse_input.jump && self.input.jump,
            InputType::Sprint => !self.previouse_input.sprint && self.input.sprint,
            InputType::TurnHorizontal => {
                self.input.turn_horizontal - self.previouse_input.turn_horizontal > THRESHOLD
            }
            InputType::TurnVertical => {
                self.input.turn_vertical - self.previouse_input.turn_vertical > THRESHOLD
            }
            InputType::Special => !self.previouse_input.special && self.input.special,
            InputType::Fire => !self.previouse_input.fire && self.input.fire,
        }
    }

    pub fn is_input_changed_to_true_and_set_to_false(&mut self, input_type: InputType) -> bool {
        match input_type {
            InputType::Up => {
                let val = !self.previouse_input.up && self.input.up;
                if val {
                    self.previouse_input.up = true;
                }
                val
            }
            InputType::Down => {
                let val = !self.previouse_input.down && self.input.down;
                if val {
                    self.previouse_input.down = true;
                }
                val
            }
            InputType::Left => {
                let val = !self.previouse_input.left && self.input.left;
                if val {
                    self.previouse_input.left = true;
                }
                val
            }
            InputType::Right => {
                let val = !self.previouse_input.right && self.input.right;
                if val {
                    self.previouse_input.right = true;
                }
                val
            }
            InputType::Jump => {
                let val = !self.previouse_input.jump && self.input.jump;
                if val {
                    self.previouse_input.jump = true;
                }
                val
            }
            InputType::Sprint => {
                let val = !self.previouse_input.sprint && self.input.sprint;
                if val {
                    self.previouse_input.sprint = true;
                }
                val
            }
            InputType::TurnHorizontal => {
                let val =
                    self.input.turn_horizontal - self.previouse_input.turn_horizontal > THRESHOLD;
                if val {
                    self.previouse_input.turn_horizontal = 0.2;
                }
                val
            }
            InputType::TurnVertical => {
                let val = self.input.turn_vertical - self.previouse_input.turn_vertical > THRESHOLD;
                if val {
                    self.previouse_input.turn_vertical = 0.2;
                }
                val
            }
            InputType::Special => {
                let val = !self.previouse_input.special && self.input.special;
                if val {
                    self.previouse_input.special = true;
                }
                val
            }
            InputType::Fire => {
                let val = !self.previouse_input.fire && self.input.fire;
                if val {
                    self.previouse_input.fire = true;
                }
                val
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputType {
    Up,
    Down,
    Left,
    Right,
    Jump,
    Sprint,
    TurnHorizontal,
    TurnVertical,
    Special,
    Fire,
}

#[derive(Debug, Default, Serialize, Deserialize, Reflect, Clone, Copy)]
pub struct Inputs {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub sprint: bool,
    pub turn_horizontal: f32,
    pub turn_vertical: f32,
    pub special: bool,
    pub fire: bool,
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
pub struct ActorTransportData {
    pub position: Vec3,
    pub rotation: Quat,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct TransportData {
    pub players: HashMap<PlayerId, PlayerTransportData>,
    pub actors: HashMap<LinkId, ActorTransportData>,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct TransportDataResource {
    pub data: TransportData,
}

#[derive(Debug, Component, Default, Serialize, Deserialize, Clone, Copy, Reflect)]
pub struct PlayerView {
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
        app.add_event::<ChangeMapLobbyEvent>()
            .add_state::<LobbyState>()
            .add_state::<MapLoaderState>()
            .init_resource::<HostResource>()
            .init_resource::<ClientResource>()
            .add_plugins((SingleLobbyPlugins, HostLobbyPlugins, ClientLobbyPlugins));
    }
}
