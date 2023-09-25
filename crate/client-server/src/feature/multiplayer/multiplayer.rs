use bevy::prelude::*;
use renet::transport::{NetcodeTransportError, NETCODE_USER_DATA_BYTES};

use serde::{Deserialize, Serialize};

use renet::ClientId;

use std::collections::HashMap;

pub const PROTOCOL_ID: u64 = 7;

#[derive(Resource, Default, Debug)]
pub struct TransportData {
  // let mut players: HashMap<ClientId, ([f32; 3], [f32; 4])> = HashMap::new();
  pub data: HashMap<ClientId, (
    [f32; 3], // position
    [f32; 4], // rotation
    [f32; 4], // tied camera rotation
  )>,
}

#[derive(Debug, Default, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
  pub turn_left: bool,
  pub turn_right: bool,
}

#[derive(Debug, Component)]
pub struct Player {
  pub id: ClientId,
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
}

/// player view direction in global spase
#[derive(Debug, Component)]
#[derive(Default)]
pub struct PlayerViewDirrection(pub Quat);



#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
  InitConnection { id: ClientId },
  PlayerConnected { id: ClientId, color: Color },
  PlayerDisconnected { id: ClientId },
}

pub fn panic_on_error_system(mut renet_errors: EventReader<NetcodeTransportError>) {
  for error in renet_errors.iter() {
    log::error!("{error:?}");
    // panic!();
  }
}

#[derive(Resource)]
pub struct Connection {
  pub initiate_connection: bool, 
}

impl Default for Connection {
  fn default() -> Self {
    Self {
      initiate_connection: false,
    }
  }
}

pub struct Error(String);

#[derive(Resource)]
pub struct Username(pub String); 

impl Default for Username {
  fn default() -> Self {
    Self("noname".to_string())
  }
}

// impl std::ops::Deref for Username {
//     fn deref(&self) -> String {
//         &self.0
//     }
// }

impl Username {
  fn to_netcode_data(&self) -> Result<[u8; NETCODE_USER_DATA_BYTES], Error> {
      let mut data = [0u8; NETCODE_USER_DATA_BYTES];
      if self.0.len() > NETCODE_USER_DATA_BYTES - 8 {
          return Err(Error("Your username to long".to_string()));
      }
      data[0..8].copy_from_slice(&(self.0.len() as u64).to_le_bytes());
      data[8..self.0.len() + 8].copy_from_slice(self.0.as_bytes());

      Ok(data)
  }

  fn from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> String {
    let mut buffer = [0u8; 8];
    buffer.copy_from_slice(&user_data[0..8]);
    let mut len = u64::from_le_bytes(buffer) as usize;
    len = len.min(NETCODE_USER_DATA_BYTES - 8);
    let data = user_data[8..len + 8].to_vec();
    let username = String::from_utf8(data).unwrap(); // TODO 

    username 
  }
}
