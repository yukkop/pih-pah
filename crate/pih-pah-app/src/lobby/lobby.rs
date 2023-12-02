use bevy::math::{Quat, Vec3};
use bevy::prelude::{Color, Component, Entity, Resource, States};
use bevy_renet::RenetServerPlugin;
use bevy_renet::transport::NetcodeServerPlugin;
use renet::ClientId;
use serde::{Deserialize, Serialize};
use bevy::app::{App, Plugin};
use std::collections::HashMap;
use crate::lobby::single::SingleLobbyPlugins;

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

#[derive(Debug, Default, Resource)]
pub struct MultiplayerResource {
    pub address: Option<String>,
}

pub struct LobbyPlugins;

impl Plugin for LobbyPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_state::<LobbyState>()
            .init_resource::<MultiplayerResource>()
            .add_plugins((RenetServerPlugin, NetcodeServerPlugin))
            .add_plugins((SingleLobbyPlugins, HostLobbyPlugins));
    }
}

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<ClientId, PlayerData>,
    pub players_seq: usize,
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
    pub id: ClientId,
}

#[derive(Resource, Default, Debug/*, Serialize, Deserialize*/)]
pub struct PlayerTransportData {
    pub position: Vec3,
    pub rotation: Quat,
    pub tied_camera_rotation: Quat,
}

#[derive(Resource, Default, Debug/*, Serialize, Deserialize*/)]
pub struct TransportData {
    pub data: HashMap<ClientId, PlayerTransportData>,
}

#[derive(Debug, Component, Default)]
pub struct PlayerViewDirection(pub Quat);
