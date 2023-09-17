use bevy::prelude::*;
use renet::transport::NetcodeTransportError;

use serde::{Deserialize, Serialize};

use renet::ClientId;

use std::collections::HashMap;

pub const PROTOCOL_ID: u64 = 7;

#[derive(Resource, Default, Debug)]
pub struct TransportData {
  // let mut players: HashMap<ClientId, ([f32; 3], [f32; 4])> = HashMap::new();
  pub data: HashMap<ClientId, ([f32; 3], [f32; 4])>,
}

#[derive(Debug, Default, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
}

#[derive(Debug, Component)]
pub struct Player {
  pub id: ClientId,
}

#[derive(Debug, Default, Resource)]
pub struct Lobby {
  pub players: HashMap<ClientId, PlayerData>,
}

#[derive(Debug)]
pub struct PlayerData {
  pub entity: Entity,
  pub view_dirrection: PlayerViewDirrection,
}

/// player view direction in global spase
#[derive(Debug, Component)]
pub struct PlayerViewDirrection(Vec3);

impl Default for PlayerViewDirrection {
  fn default() -> Self {
    // forward
    Self(Vec3::NEG_Z)
  }
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
  InitConnection { id: ClientId },
  PlayerConnected { id: ClientId },
  PlayerDisconnected { id: ClientId },
}

pub fn panic_on_error_system(mut renet_errors: EventReader<NetcodeTransportError>) {
  for error in renet_errors.iter() {
    log::error!("ERROR: {error:?}");
    panic!();
  }
}
