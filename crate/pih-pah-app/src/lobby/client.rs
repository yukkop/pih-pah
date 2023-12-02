use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::app::{App, Plugin, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::schedule::{OnExit, Condition};
use bevy::ecs::system::{Query, Res, ResMut};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec3;
use bevy::prelude::{Color, Commands, in_state, IntoSystemConfigs, OnEnter};
use bevy_renet::RenetClientPlugin;
use bevy_renet::transport::NetcodeClientPlugin;
use renet::transport::{ClientAuthentication, NetcodeClientTransport};
use renet::{ClientId, ConnectionConfig, RenetClient, DefaultChannel};
use crate::character::{spawn_character, spawn_tied_camera, TiedCamera, spawn_character_shell};
use crate::lobby::LobbyState;
use crate::world::Me;

use super::{PlayerInput, PROTOCOL_ID, ClientResource, Username};

pub struct ClientLobbyPlugins;

impl Plugin for ClientLobbyPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((RenetClientPlugin, NetcodeClientPlugin))
            .add_systems(OnEnter(LobbyState::Client),
                        (setup, new_renet_client))
            .add_systems(
                Update,
                client_send_input
                    .run_if(in_state(LobbyState::Client)
                    .and_then(bevy_renet::client_connected())))
            .add_systems(OnExit(LobbyState::Client),
                        teardown);
    }
}

pub fn new_renet_client(settings: Res<ClientResource>, mut commands: Commands) {
      commands.insert_resource(RenetClient::new(ConnectionConfig::default()));
      let server_addr = settings.address.clone().unwrap().parse().unwrap();
      let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
      let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
      let client_id = current_time.as_millis() as u64;
  
      let username_netcode = match Username(settings.username.clone().unwrap().clone()).to_netcode_data() {
        Ok(bytes) => Some(bytes),
        Err(_) => None,
      };
  
      let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: username_netcode,
      };
  
      commands
        .insert_resource(NetcodeClientTransport::new(current_time, authentication, socket).unwrap());
}

pub fn client_send_input(
    mut player_input_query: Query<&mut PlayerInput, With<Me>>,
    mut client: ResMut<RenetClient>
) {
    if let Ok(player_input) = player_input_query.get_single_mut() {
        let input_message = bincode::serialize(&*player_input).unwrap();
  
        client.send_message(DefaultChannel::ReliableOrdered, input_message);
    }
}

fn setup(
    mut commands: Commands,
) {
    // me
    let a = Vec3::new(0., 10., 0.);
    let entity = commands
        .spawn_character_shell(ClientId::from_raw(0), Color::RED, a).insert(Me).id();
    commands.spawn_tied_camera(entity);
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