use std::net::UdpSocket;
use std::time::SystemTime;

use crate::character::{spawn_character_shell, spawn_tied_camera, TiedCamera};
use crate::lobby::{LobbyState, PlayerId};
use crate::province::ProvinceState;
use crate::world::{LinkId, Me};
use bevy::app::{App, Plugin, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::schedule::{Condition, NextState, OnExit};
use bevy::ecs::system::{Query, Res, ResMut, Resource};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec3;
use bevy::prelude::{in_state, Commands, IntoSystemConfigs, OnEnter};
use bevy::transform::components::Transform;
use bevy_renet::transport::NetcodeClientPlugin;
use bevy_renet::RenetClientPlugin;
use renet::transport::{ClientAuthentication, NetcodeClientTransport};
use renet::{ClientId, ConnectionConfig, DefaultChannel, RenetClient};

#[derive(Default, Debug, Resource)]
pub struct OwnId(Option<ClientId>);

use super::{
    ClientResource, Lobby, PlayerData, PlayerInput, ServerMessages, TransportDataResource,
    Username, PROTOCOL_ID,
};

pub struct ClientLobbyPlugins;

impl Plugin for ClientLobbyPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((RenetClientPlugin, NetcodeClientPlugin))
            .add_systems(OnEnter(LobbyState::Client), (setup, new_renet_client))
            .add_systems(
                Update,
                (client_send_input, client_sync_players)
                    .run_if(in_state(LobbyState::Client).and_then(bevy_renet::client_connected())),
            )
            .add_systems(OnExit(LobbyState::Client), teardown);
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

    let username_netcode =
        match Username(settings.username.clone().unwrap().clone()).to_netcode_data() {
            Ok(bytes) => Some(bytes),
            Err(_) => None,
        };

    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: username_netcode,
    };

    commands.insert_resource(
        NetcodeClientTransport::new(current_time, authentication, socket).unwrap(),
    );
}

pub fn client_send_input(
    mut player_input_query: Query<&mut PlayerInput, With<Me>>,
    mut client: ResMut<RenetClient>,
) {
    if let Ok(player_input) = player_input_query.get_single_mut() {
        let input_message = bincode::serialize(&*player_input).unwrap();

        client.send_message(DefaultChannel::ReliableOrdered, input_message);
    }
}

fn setup(mut commands: Commands) {
    // me
    // let a = Vec3::new(0., 10., 0.);
    // let entity = commands
    //     .spawn_character_shell(ClientId::from_raw(0), Color::RED, a).insert(Me).id();
    // commands.spawn_tied_camera(entity);
    commands.init_resource::<Lobby>();
    commands.init_resource::<OwnId>();
    commands.init_resource::<TransportDataResource>();
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
    commands.remove_resource::<OwnId>();
    commands.remove_resource::<TransportDataResource>();
}

#[allow(clippy::too_many_arguments)]
pub fn client_sync_players(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut transport_data: ResMut<TransportDataResource>,
    mut lobby: ResMut<Lobby>,
    mut own_id: ResMut<OwnId>,
    mut tied_camera_query: Query<&mut Transform, With<TiedCamera>>,
    mut next_state_province: ResMut<NextState<ProvinceState>>,
    lincked_obj_query: Query<(Entity, &LinkId)>,
) {
    // player existence manager
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::InitConnection { id, province_state } => {
                next_state_province.set(province_state);
                if own_id.0.is_some() {
                    panic!("Yeah, I knew it. The server only had to initialize me once. Redo it, you idiot.");
                } else {
                    *own_id = OwnId(Some(id));
                }
            }
            ServerMessages::ChangeProvince { province_state } => {
                next_state_province.set(province_state);
            }
            ServerMessages::PlayerConnected {
                id: player_id,
                color,
                username,
            } => {
                let name = "noname";

                let player_entity = commands.spawn_character_shell(color, Vec3::ZERO).id();
                if let PlayerId::Client(id) = player_id {
                    if Some(id) == own_id.0 {
                        commands.entity(player_entity).insert(Me);
                        commands.spawn_tied_camera(player_entity);
                        log::info!("{name} ({id}), welcome.");
                    } else {
                        log::info!("Player {} ({}) connected.", name, id);
                    }
                } else {
                    log::info!("Host {} ({:?}).", name, player_id);
                }

                lobby.players.insert(
                    player_id,
                    PlayerData {
                        entity: player_entity,
                        color,
                        username,
                    },
                );
            }
            ServerMessages::PlayerDisconnected { id } => {
                let name = "noname";

                log::info!("Player {} ({:?}) disconnected.", name, id);
                if let Some(player_data) = lobby.players.remove(&id) {
                    commands.entity(player_data.entity).despawn();
                }
            }
        }
    }

    // movements
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        transport_data.data = bincode::deserialize(&message).unwrap();
        for (player_id, data) in transport_data.data.players.iter() {
            if let Some(player_data) = lobby.players.get(player_id) {
                let transform = Transform {
                    translation: data.position,
                    rotation: data.rotation,
                    ..Default::default()
                };
                // TODO: why transform to default?
                commands.entity(player_data.entity).insert(transform);
                if let PlayerId::Client(id) = player_id {
                    if Some(id) == own_id.0.as_ref() {
                        if let Ok(mut camera_transform) = tied_camera_query.get_single_mut() {
                            camera_transform.translation = transform.translation;
                            camera_transform.rotation = data.tied_camera_rotation;
                        }
                    }
                }
            }
        }

        for (link_id, data) in transport_data.data.objects.iter() {
            for (entity, id) in lincked_obj_query.iter() {
                if id == link_id {
                    let transform = Transform {
                        translation: data.position,
                        rotation: data.rotation,
                        ..Default::default()
                    };
                    commands.entity(entity).insert(transform);
                }
            }
        }
    }
}
