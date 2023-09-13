pub mod lib {
  pub mod utils;

  use renet::transport::NetcodeTransportError;
  use bevy::prelude::*;

  use renet::ClientId;

  use std::collections::HashMap;

  use serde::{Deserialize, Serialize};

  pub const PROTOCOL_ID: u64 = 7;
  pub const PLAYER_MOVE_SPEED: f32 = 1.0;

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

  pub fn move_players_system(mut query: Query<(&mut Transform, &PlayerInput)>, time: Res<Time>) {
      for (mut transform, input) in query.iter_mut() {
          let x = (input.right as i8 - input.left as i8) as f32;
          let y = (input.down as i8 - input.up as i8) as f32;
          transform.translation.x += x * PLAYER_MOVE_SPEED * time.delta().as_secs_f32();
          transform.translation.z += y * PLAYER_MOVE_SPEED * time.delta().as_secs_f32();
      }
  }

  pub fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
      for e in renet_error.iter() {
          dbg!(e);
          panic!("{}", e);
      }
  }
}
