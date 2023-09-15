use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_xpbd_3d::prelude::*;
use pih_pah::lib::netutils::{is_http_address, is_ip_with_port};
use pih_pah::lib::{
    panic_on_error_system, Lobby, Player, PlayerInput, ServerMessages, TransportData,
    PROTOCOL_ID,
};
use pih_pah::feature::lobby::spawn_player;
use pih_pah::feature::lobby::LobbyMinimalPlugins;
use pih_pah::feature::ui::FpsPlugins;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_renet::{
    renet::{
        transport::{ServerAuthentication, ServerConfig},
        ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use renet::{transport::NetcodeServerTransport, ClientId};

use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};

struct Data {
  // let mut players: HashMap<ClientId, ([f32; 3], [f32; 4])> = HashMap::new();
  data: HashMap<ClientId, ([f32; 3], [f32; 4])>
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();

    log::info!("{}", args.len());
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
        _ => panic!("Invalid argument, must be an HTTP address or an IP with port."),
    };

    let mut is_not_debug = true;
    if args.len() > 2 {
      is_not_debug = match args[2].as_str() {
        "terminal" => true,
        "debug" => false,
        _ => panic!("Invalid argument, must be an HTTP address or an IP with port."),
      }
    }

    let mut app = App::new();
      app.init_resource::<Lobby>();


use bevy::window::*;

    if is_not_debug {
      app.add_plugins((
         MinimalPlugins,
       ));
    } else {
      app.add_plugins((
         DefaultPlugins.set(WindowPlugin {
           primary_window: Window { 
             title: "Game of Life".to_string(),
             // this is need's for stable fps
             present_mode: PresentMode::AutoNoVsync,
             ..default()
           }.into(),
         ..default()
         }),
         EguiPlugin,
         FpsPlugins,
         LogDiagnosticsPlugin::default(),
         FrameTimeDiagnosticsPlugin::default()
       ));
      app.add_plugins(WorldInspectorPlugin::default());
    }

    app.add_plugins((
                     LobbyMinimalPlugins,
                     PhysicsPlugins::default(),
                     RenetServerPlugin,
                     NetcodeServerPlugin));
    let (server, transport) = new_renet_server(server_addr.to_string());
    app.insert_resource(server);
    app.insert_resource(transport);
    //some about connection
    app.init_resource::<TransportData>();
  
    app.add_systems(
        Update,
        (
            server_update_system,
            server_sync_players,
        )
            .run_if(resource_exists::<RenetServer>()),
    );

    app.add_systems(Update, panic_on_error_system);

    app.run();
}

// fn print_time(time: Res<Time>) {
//     println!("Current time: {:?}", time.seconds_since_startup());
// }

fn new_renet_server(addr: String) -> (RenetServer, NetcodeServerTransport) {
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

fn server_sync_players(mut server: ResMut<RenetServer>, mut data: ResMut<TransportData>, query: Query<(&Position, &Rotation, &Player)>) {
    // let mut players: HashMap<ClientId, [[f32; 3]; 2]> = HashMap::new();
    for (position, rotation, player) in query.iter() {
        data.data.insert(player.id, (position.0.into(), rotation.0.into()));
    }

    let sync_message = bincode::serialize(&data.data).unwrap();
    server.broadcast_message(DefaultChannel::Unreliable, sync_message);

    data.data.clear();
}
