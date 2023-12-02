use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::app::{App, Plugin, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::schedule::OnExit;
use bevy::ecs::system::{Query, Res};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec3;
use bevy::prelude::{Color, Commands, in_state, IntoSystemConfigs, OnEnter};
use renet::transport::{NetcodeServerTransport, ServerConfig, ServerAuthentication};
use renet::{ClientId, RenetServer, ConnectionConfig};
use crate::character::{spawn_character, spawn_tied_camera, TiedCamera};
use crate::lobby::LobbyState;
use crate::world::Me;

use super::{PlayerInput, Lobby, TransportData, PROTOCOL_ID, MultiplayerResource};

pub struct HostLobbyPlugins;

impl Plugin for HostLobbyPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LobbyState::Host),
                        setup);
        app.add_systems(Update,
                        update.run_if(in_state(LobbyState::Host)));
        app.add_systems(OnExit(LobbyState::Host),
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
    multiplayer_resource: Res<MultiplayerResource>,
) {
    // me
    let a = Vec3::new(0., 10., 0.);
    let entity = commands
        .spawn_character(ClientId::from_raw(0), Color::RED, a).insert(Me).id();
    commands.spawn_tied_camera(entity);

    // server
    commands.init_resource::<Lobby>();
    commands.init_resource::<TransportData>();

    let (server, transport) = new_renet_server(multiplayer_resource.address.clone().unwrap().as_str());
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
}