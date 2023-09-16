use bevy::prelude::*;
use renet::transport::NetcodeTransportError;

use serde::{Deserialize, Serialize};

use crate::feature::lobby::spawn_player;
use bevy_renet::{
    renet::{
        transport::{ServerAuthentication, ServerConfig},
        ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use bevy_xpbd_3d::prelude::*;
use renet::{transport::NetcodeServerTransport, ClientId};

use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};

pub const PROTOCOL_ID: u64 = 7;
pub const PLAYER_MOVE_SPEED: f32 = 0.07;

pub const PLAYER_SIZE: f32 = 1.0;
pub const PLAYER_SPAWN_POINT: Vec3 = Vec3::new(0., 10., 0.);

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
    pub players: HashMap<ClientId, Entity>,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: ClientId },
    PlayerDisconnected { id: ClientId },
}

pub fn panic_on_error_system(mut renet_errors: EventReader<NetcodeTransportError>) {
    for error in renet_errors.iter() {
        log::error!("ERROR: {error:?}");
        panic!();
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

pub fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                log::info!("Player {} connected.", client_id);
                // Spawn player cube
                let player_entity = commands.spawn_player(*client_id).id();

                // We could send an InitState with all the players id and positions for the client
                // but this is easier to do.
                for &player_id in lobby.players.keys() {
                    let message =
                        bincode::serialize(&ServerMessages::PlayerConnected { id: player_id })
                            .unwrap();
                    server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
                }

                lobby.players.insert(*client_id, player_entity);

                let message =
                    bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                log::info!("Player {} disconnected: {}", client_id, reason);
                if let Some(player_entity) = lobby.players.remove(client_id) {
                    commands.entity(player_entity).despawn();
                }

                let message =
                    bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let player_input: PlayerInput = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                commands.entity(*player_entity).insert(player_input);
            }
        }
    }
}

pub fn server_sync_players(
    mut server: ResMut<RenetServer>,
    mut data: ResMut<TransportData>,
    query: Query<(&Position, &Rotation, &Player)>,
) {
    // let mut players: HashMap<ClientId, [[f32; 3]; 2]> = HashMap::new();
    for (position, rotation, player) in query.iter() {
        data.data
            .insert(player.id, (position.0.into(), rotation.0.into()));
    }

    let sync_message = bincode::serialize(&data.data).unwrap();
    server.broadcast_message(DefaultChannel::Unreliable, sync_message);

    data.data.clear();
}
