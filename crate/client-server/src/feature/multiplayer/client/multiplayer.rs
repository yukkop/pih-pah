use bevy::prelude::*;

use crate::feature::lobby::client::camera_switch;
use crate::feature::lobby::client::spawn_client_side_player;
use crate::feature::lobby::client::{spawn_tied_camera, TiedCamera};
use crate::feature::multiplayer::{
  Lobby, Connection, Username, PlayerData, PlayerInput, ServerMessages, TransportData, PROTOCOL_ID,
};
use bevy_renet::{
  renet::{transport::ClientAuthentication, ConnectionConfig, DefaultChannel, RenetClient},
  transport::NetcodeClientPlugin,
  RenetClientPlugin,
};
use renet::{transport::NetcodeClientTransport, ClientId};

use std::{
  net::UdpSocket,
  time::SystemTime
};

#[derive(Default, Debug, Resource)]
pub struct OwnId(Option<ClientId>);

pub struct MultiplayerPlugins;

impl Plugin for MultiplayerPlugins {
  fn build(&self, app: &mut App) {
    app.init_resource::<Lobby>();
    app.init_resource::<TransportData>();

    // if RenetClient no
    app.add_plugins(RenetClientPlugin);
    app.add_plugins(NetcodeClientPlugin);
    app.init_resource::<PlayerInput>();
    app.init_resource::<OwnId>();

    app.init_resource::<Username>();
    app.init_resource::<Connection>();
    app.add_event::<InitConnectionEvent>();

    app.add_systems(Update, new_renet_client);
    app.add_systems(
      Update,
      (
        player_input,
        camera_switch, // TODO maybe separate
        client_send_input,
        client_sync_players,
      )
        .run_if(bevy_renet::transport::client_connected()),
    );
  }
}

#[derive(Default, Event)]
pub struct InitConnectionEvent{
  pub addr: String,
  pub username: String,
}

fn if_initiate_connection(
  connection: Res<Connection>,
) -> bool {
  println!("{}", connection.initiate_connection);
  if connection.initiate_connection {
    return true;
  }
  false
}
pub fn new_renet_client(
  mut ev: EventReader<InitConnectionEvent>,
  mut commands: Commands,
) {
 for settings in ev.iter() {
    commands.insert_resource(RenetClient::new(ConnectionConfig::default()));
    let server_addr = settings.addr.parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let current_time = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap();
    let client_id = current_time.as_millis() as u64;

    let username_netcode = match Username(settings.username.clone()).to_netcode_data() {
      Ok(bytes) => Some(bytes),
      Err(_) => None
    };

    let authentication = ClientAuthentication::Unsecure {
      client_id,
      protocol_id: PROTOCOL_ID,
      server_addr,
      user_data: username_netcode,
    };

    commands.insert_resource(NetcodeClientTransport::new(current_time, authentication, socket).unwrap());
  }
}

pub fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
  let input_message = bincode::serialize(&*player_input).unwrap();

  client.send_message(DefaultChannel::ReliableOrdered, input_message);
}

pub fn client_sync_players(
  mut commands: Commands,
  _meshes: ResMut<Assets<Mesh>>,
  _materials: ResMut<Assets<StandardMaterial>>,
  mut client: ResMut<RenetClient>,
  mut transport_data: ResMut<TransportData>,
  mut lobby: ResMut<Lobby>,
  mut own_id: ResMut<OwnId>,
  mut tied_camera_query: Query<&mut Transform, With<TiedCamera>>,
) {
  // player existence manager
  while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
    let server_message = bincode::deserialize(&message).unwrap();
    match server_message {
      ServerMessages::InitConnection { id } => {
        if own_id.0.is_some() {
          panic!("Yeah, I knew it. The server only had to initialize me once. Redo it, you idiot.");
        } else {
          *own_id = OwnId(Some(id));
        }
      }
      ServerMessages::PlayerConnected { id, color, username } => {
        let name = "noname";

        let player_entity = commands.spawn_client_side_player(color).id();
        if Some(id) == own_id.0 {
          commands.spawn_tied_camera();
          log::info!("{name} ({id}), welcome.");
        }
        else {
          log::info!("Player {} ({}) connected.", name, id);
        }

        lobby.players.insert(
          id,
          PlayerData {
            entity: player_entity,
            color,
            username
          },
        );
      }
      ServerMessages::PlayerDisconnected { id } => {
        let name = "noname";

        log::info!("Player {} ({}) disconnected.", name, id);
        if let Some(player_data) = lobby.players.remove(&id) {
          commands.entity(player_data.entity).despawn();
        }
      }
    }
  }

  // players movements
  while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
    transport_data.data = bincode::deserialize(&message).unwrap();
    for (player_id, data) in transport_data.data.iter() {
      if let Some(player_data) = lobby.players.get(player_id) {
        let transform = Transform {
          translation: (data.position).into(),
          rotation: Quat::from_array(data.rotation),
          ..Default::default()
        };
        commands.entity(player_data.entity).insert(transform);
        if Some(player_id) == own_id.0.as_ref() {
          if let Ok(mut camera_transform) = tied_camera_query.get_single_mut() {
            camera_transform.translation = transform.translation;
            camera_transform.rotation = Quat::from_array(data.tied_camera_rotation);
          }
        }
      }
    }
  }
}

pub fn player_input(keyboard_input: Res<Input<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
  player_input.left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
  player_input.right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
  player_input.up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
  player_input.down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
  player_input.turn_left = keyboard_input.pressed(KeyCode::Q);
  player_input.turn_right = keyboard_input.pressed(KeyCode::E);
}
