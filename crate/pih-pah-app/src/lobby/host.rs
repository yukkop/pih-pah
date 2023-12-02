use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::app::{App, Plugin, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventReader;
use bevy::ecs::query::{With, Without};
use bevy::ecs::schedule::OnExit;
use bevy::ecs::system::{Query, Res, ResMut};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec3;
use bevy::prelude::{Color, Commands, in_state, IntoSystemConfigs, OnEnter};
use bevy_renet::RenetServerPlugin;
use bevy_renet::transport::NetcodeServerPlugin;
use bevy_xpbd_3d::components::{Position, Rotation};
use renet::transport::{NetcodeServerTransport, ServerConfig, ServerAuthentication};
use renet::{ClientId, RenetServer, ConnectionConfig, ServerEvent, DefaultChannel};
use crate::character::{spawn_character, spawn_tied_camera, TiedCamera};
use crate::lobby::{LobbyState, ServerMessages, Username, PlayerData, PlayerId};
use crate::world::Me;

use super::{PlayerInput, Lobby, TransportData, PROTOCOL_ID, HostResource, PlayerViewDirection, Character, PlayerTransportData, lobby};

pub struct HostLobbyPlugins;

impl Plugin for HostLobbyPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((RenetServerPlugin, NetcodeServerPlugin))
            .add_systems(OnEnter(LobbyState::Host),
                        setup)
            .add_systems(Update,
                        (update, server_update_system, server_sync_players).run_if(in_state(LobbyState::Host)))
            .add_systems(OnExit(LobbyState::Host),
                        teardown);
    }
}

pub fn new_renet_server(addr: &str) -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(ConnectionConfig::default());
  
    let public_addr = addr.parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap();
    let server_config = ServerConfig {
      current_time,
      max_clients: 64,
      protocol_id: PROTOCOL_ID,
      public_addresses: vec![public_addr],
      authentication: ServerAuthentication::Unsecure,
    };
  
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
  
    (server, transport)
}

fn setup(
    mut commands: Commands,
    host_resource: Res<HostResource>,
) {
    // me

    // server
    commands.init_resource::<TransportData>();

    let mut lobby = Lobby::default();
    lobby.players_seq += 1;
    let color = generate_player_color(lobby.players_seq as u32);

    let a = Vec3::new(0., 10., 0.);
    let player_entity = commands
        .spawn_character(PlayerId::Host, color, a).insert(Me).id();
    commands.spawn_tied_camera(player_entity);

    lobby.players.insert(
            PlayerId::Host,
            PlayerData {
              entity: player_entity,
              color,
              username: host_resource.username.clone().unwrap(),
            },
          );
    commands.insert_resource(lobby);

    let (server, transport) = new_renet_server(host_resource.address.clone().unwrap().as_str());
    commands.insert_resource(server);
    commands.insert_resource(transport);
}

fn update(
) {

}

fn teardown(
    mut commands: Commands,
    tied_camera_query: Query<Entity, With<TiedCamera>>,
    char_query: Query<Entity, With<PlayerInput>>, 
) {
    for entity in tied_camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in char_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<Lobby>();
    commands.remove_resource::<TransportData>();
}

pub fn generate_player_color(player_number: u32) -> Color {
    let golden_angle = 137.5;
    let hue = (golden_angle * player_number as f32) % 360.0;
    Color::hsl(hue, 1.0, 0.5)
  }
  

pub fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
    transport: Res<NetcodeServerTransport>,
  ) {
    for event in server_events.read() {
      match event {
        ServerEvent::ClientConnected { client_id } => {
          log::info!("Player {} connected.", client_id);
  
          // TODO remove
          let message =
            bincode::serialize(&ServerMessages::InitConnection { id: *client_id }).unwrap();
          server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
  
          lobby.players_seq += 1;
          let color = generate_player_color(lobby.players_seq as u32);
  
          // Spawn player cube
          let player_entity = commands.spawn_character(PlayerId::Client(*client_id), color, Vec3::new(0., 10., 0.)).id();
  
          // We could send an InitState with all the players id and positions for the multiplayer
          // but this is easier to do.
          for (player_id, player_data) in &lobby.players {
                let message = bincode::serialize(&ServerMessages::PlayerConnected {
                id: *player_id,
                color: player_data.color,
                username: player_data.username.clone(),
                })
                .unwrap();
                server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
            
          }
  
          let data = transport.user_data(*client_id).unwrap();
          let username = match Username::from_user_data(&data) {
            Ok(name) => name,
            Err(_) => "@corapted@".to_string(),
          };
          // let username = "noname".to_string();
  
          lobby.players.insert(
            PlayerId::Client(*client_id),
            PlayerData {
              entity: player_entity,
              color,
              username: username.clone(),
            },
          );
  
          let message = bincode::serialize(&ServerMessages::PlayerConnected {
            id: PlayerId::Client(*client_id),
            color,
            username,
          })
          .unwrap();
          server.broadcast_message(DefaultChannel::ReliableOrdered, message);
        }
        ServerEvent::ClientDisconnected { client_id, reason } => {
          log::info!("Player {} disconnected: {}", client_id, reason);
          if let Some(player_data) = lobby.players.remove(&PlayerId::Client(*client_id)) {
            commands.entity(player_data.entity).despawn();
          }
  
          let message =
            bincode::serialize(&ServerMessages::PlayerDisconnected { id: PlayerId::Client(*client_id) }).unwrap();
          server.broadcast_message(DefaultChannel::ReliableOrdered, message);
        }
      }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
          let player_input: PlayerInput = bincode::deserialize(&message).unwrap();
          if let Some(player_data) = lobby.players.get(&PlayerId::Client(client_id)) {
            commands.entity(player_data.entity).insert(player_input);
          }
        }
      }
    }

    pub fn server_sync_players(
        mut server: ResMut<RenetServer>,
        mut data: ResMut<TransportData>,
        query: Query<(&Position, &Rotation, &PlayerViewDirection, &Character)>,
      ) {
        for (position, rotation, view_direction, character) in query.iter() {
          data.data.insert(
            character.id,
            PlayerTransportData {
              position: position.0.into(),
              rotation: rotation.0.into(),
              tied_camera_rotation: view_direction.0.into(),
            },
          );
        }
      
        let sync_message = bincode::serialize(&data.data).unwrap();
        server.broadcast_message(DefaultChannel::Unreliable, sync_message);
      
        data.data.clear();
      }