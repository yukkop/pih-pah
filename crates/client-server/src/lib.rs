pub mod feature;

/// Pure reusable library modules, except for ui ones go here. Things like physics calculation or little helpers or traits or macros, etc
pub mod lib {
  pub mod netutils;
  pub mod extend_commands;

  // TODO: move all this crap where it belongs as per the new architecture
  // Look at corresponding mod.rs for explanation
  use bevy::prelude::*;
  use renet::transport::NetcodeTransportError;

  use renet::ClientId;

  use std::collections::HashMap;

  use serde::{Deserialize, Serialize};

  pub const PROTOCOL_ID: u64 = 7;
  pub const PLAYER_MOVE_SPEED: f32 = 1.0;

  pub const PLAYER_SIZE: f32 = 1.0;
  pub const PLAYER_SPAWN_POINT: Vec3 = Vec3::new(0.,10.,0.);

  #[derive(Resource, Default, Debug)]
  pub struct TransportData {
    // let mut players: HashMap<ClientId, ([f32; 3], [f32; 4])> = HashMap::new();
    pub data: HashMap<ClientId, ([f32; 3], [f32; 4])>
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
      pub players: HashMap<ClientId, Entity>,
  }

  #[derive(Debug, Serialize, Deserialize, Component)]
  pub enum ServerMessages {
      PlayerConnected { id: ClientId },
      PlayerDisconnected { id: ClientId },
  }

  pub fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
      for e in renet_error.iter() {
          dbg!(e);
          panic!("{}", e);
      }
  }
}