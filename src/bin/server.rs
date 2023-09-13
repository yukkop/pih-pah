use bevy::prelude::*;
use pih_pah::lib::{Lobby, PlayerInput, Player, ServerMessages, PROTOCOL_ID, move_players_system, panic_on_error_system};
use pih_pah::lib::utils::net::{is_http_address, is_ip_with_port};

use bevy_renet::{
    renet::{
        transport::{ServerAuthentication, ServerConfig},
        ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use renet::{
    transport::NetcodeServerTransport,
    ClientId,
};

use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: "); 
        println!("\tserver.rs '<ip>:<port>'");
        println!("\tserver.rs 'http::\\\\my\\server\\address'");

        panic!("Not enough arguments.");
    }

    // Checking if the address is either an HTTP address or an IP address with port
    let server_addr = match &args[1] {
        addr if is_http_address(addr) => addr,
        addr if is_ip_with_port(addr) => addr,
        _ => panic!("Invalid argument, must be an HTTP address or an IP with port.")
    };

    let mut app = App::new();
    app.init_resource::<Lobby>();

    app.add_plugins(MinimalPlugins);
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(NetcodeServerPlugin);
    let (server, transport) = new_renet_server(server_addr.to_string());
    app.insert_resource(server);
    app.insert_resource(transport);

    app.add_systems(
        Update,
        (server_update_system, server_sync_players, move_players_system).run_if(resource_exists::<RenetServer>()),
    );
    app.add_systems(Startup, setup_server);

    app.add_systems(Update, panic_on_error_system);

    app.run();
}

fn new_renet_server(addr: String) -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(ConnectionConfig::default());

    let public_addr = addr.parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
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

fn server_update_system(
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
                println!("Player {} connected.", client_id);
                // Spawn player cube
                let player_entity = commands
                    .spawn(PbrBundle {
                        // mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        // material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                        transform: Transform::from_xyz(0.0, 0.5, 0.0),
                        ..Default::default()
                    })
                    .insert(PlayerInput::default())
                    .insert(Player { id: *client_id })
                    .id();

                // We could send an InitState with all the players id and positions for the client
                // but this is easier to do.
                for &player_id in lobby.players.keys() {
                    let message = bincode::serialize(&ServerMessages::PlayerConnected { id: player_id }).unwrap();
                    server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
                }

                lobby.players.insert(*client_id, player_entity);

                let message = bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id }).unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
                if let Some(player_entity) = lobby.players.remove(client_id) {
                    commands.entity(player_entity).despawn();
                }

                let message = bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id }).unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            let player_input: PlayerInput = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                commands.entity(*player_entity).insert(player_input);
            }
        }
    }
}

fn server_sync_players(mut server: ResMut<RenetServer>, query: Query<(&Transform, &Player)>) {
    let mut players: HashMap<ClientId, [f32; 3]> = HashMap::new();
    for (transform, player) in query.iter() {
        players.insert(player.id, transform.translation.into());
    }

    let sync_message = bincode::serialize(&players).unwrap();
    server.broadcast_message(DefaultChannel::Unreliable, sync_message);
}

fn setup_server(mut commands: Commands,
                // mut meshes: ResMut<Assets<Mesh>>,
                // mut materials: ResMut<Assets<StandardMaterial>>
                ) {
    // plane
    commands.spawn(PbrBundle {
        // mesh: meshes.add(Mesh::from(Plane::from_size(5.0))),
        // material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
