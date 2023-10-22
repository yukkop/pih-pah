use bevy::prelude::*;
use renet::transport::{NetcodeTransportError, NETCODE_USER_DATA_BYTES};

use serde::{Deserialize, Serialize};

use renet::ClientId;

use std::collections::HashMap;

pub const PROTOCOL_ID: u64 = 7;

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct PlayerTransportData {
  pub position: [f32; 3],
  pub rotation: [f32; 4],
  pub tied_camera_rotation: [f32; 4],
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct TransportData {
  pub data: HashMap<ClientId, PlayerTransportData>,
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
  pub username: String,
}

/// player view direction in global spase
#[derive(Debug, Component, Default)]
pub struct PlayerViewDirrection(pub Quat);

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
  InitConnection {
    id: ClientId,
  },
  PlayerConnected {
    id: ClientId,
    color: Color,
    username: String,
  },
  PlayerDisconnected {
    id: ClientId,
  },
}

pub fn panic_on_error_system(mut renet_errors: EventReader<NetcodeTransportError>) {
  for error in renet_errors.iter() {
    log::error!("{error:?}");
    // panic!();
  }
}

#[derive(Resource, Default)]
pub struct Connection {
  pub initiate_connection: bool,
}

#[derive(Debug)]
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
  pub fn to_netcode_data(&self) -> Result<[u8; NETCODE_USER_DATA_BYTES], Error> {
    let mut data = [0u8; NETCODE_USER_DATA_BYTES];
    if self.0.len() > NETCODE_USER_DATA_BYTES - 8 {
      let err = Error("Your username to long".to_string());
      log::error!("{:?}", err);
      return Err(err);
    }
    data[0..8].copy_from_slice(&(self.0.len() as u64).to_le_bytes());
    data[8..self.0.len() + 8].copy_from_slice(self.0.as_bytes());

    Ok(data)
  }

  // pub fn to_netcode_user_data(&self) -> [u8; NETCODE_USER_DATA_BYTES] {
  //   let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
  //   if self.0.len() > NETCODE_USER_DATA_BYTES - 8 {
  //       panic!("Username is too big");
  //   }
  //   user_data[0..8].copy_from_slice(&(self.0.len() as u64).to_le_bytes());
  //   user_data[8..self.0.len() + 8].copy_from_slice(self.0.as_bytes());
  //
  //   user_data
  // }

  // pub fn from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> Self {
  //     let mut buffer = [0u8; 8];
  //     buffer.copy_from_slice(&user_data[0..8]);
  //     let mut len = u64::from_le_bytes(buffer) as usize;
  //     len = len.min(NETCODE_USER_DATA_BYTES - 8);
  //     let data = user_data[8..len + 8].to_vec();
  //     let username = String::from_utf8(data).unwrap();
  //     Self(username)
  //   }

  pub fn from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> Result<String, Error> {
    let mut buffer = [0u8; 8];
    buffer.copy_from_slice(&user_data[0..8]);
    let mut len = u64::from_le_bytes(buffer) as usize;
    len = len.min(NETCODE_USER_DATA_BYTES - 8);
    let data = user_data[8..len + 8].to_vec();
    let username = String::from_utf8(data).map_err(|err| {
      log::error!("{:?}", err);
      Error(err.to_string())
    })?;

    Ok(username)
  }
}
