use bevy::prelude::{shape::Plane, *};
use pih_pah::lib::music::plugin::MusicPlugins;
use pih_pah::lib::ui::UiPlugins;
use pih_pah::lib::utils::net::{is_http_address, is_ip_with_port};
use pih_pah::lib::{panic_on_error_system, Lobby, PlayerInput, ServerMessages, PROTOCOL_ID};
use pih_pah::lobby::LobbyDefaultPlugins;
use pih_pah::lib::{PLAYER_SIZE, PLAYER_SPAWN_POINT};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_renet::{
    renet::{transport::ClientAuthentication, ConnectionConfig, DefaultChannel, RenetClient},
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use renet::{transport::NetcodeClientTransport, ClientId};

use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};

fn new_renet_client(addr: String) -> (RenetClient, NetcodeClientTransport) {
    let client = RenetClient::new(ConnectionConfig::default());
    log::info!("{}", addr);
    let server_addr = addr.parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    (client, transport)
}

fn main() {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: ");
        println!("\tclient.rs '<ip>:<port>'");
        println!("\tclient.rs 'http::\\\\my\\server\\address'");

        panic!("Not enough arguments.");
    }

    // Checking if the address is either an HTTP address or an IP address with port
    let server_addr = match &args[1] {
        addr if is_http_address(addr) => addr,
        addr if is_ip_with_port(addr) => addr,
        _ => panic!("Invalid argument, must be an HTTP address or an IP with port."),
    };

    let mut app = App::new();
    app.init_resource::<Lobby>();

    app.add_plugins((DefaultPlugins, MusicPlugins, UiPlugins, LobbyDefaultPlugins));
    app.add_plugins(WorldInspectorPlugin::default());
    app.add_plugins(RenetClientPlugin);
    app.add_plugins(NetcodeClientPlugin);
    app.init_resource::<PlayerInput>();
    let (client, transport) = new_renet_client(server_addr.to_string());
    app.insert_resource(client);
    app.insert_resource(transport);

    app.add_systems(
        Update,
        (player_input, client_send_input, client_sync_players)
            .run_if(bevy_renet::transport::client_connected()),
    );

    app.add_systems(Update, panic_on_error_system);

    app.run();
}

fn player_input(keyboard_input: Res<Input<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    player_input.left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
    player_input.right =
        keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
    player_input.up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
    player_input.down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
}

fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
    let input_message = bincode::serialize(&*player_input).unwrap();

    client.send_message(DefaultChannel::ReliableOrdered, input_message);
}

fn client_sync_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
) {
  // player existence manager
  while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
      let server_message = bincode::deserialize(&message).unwrap();
      match server_message {
          ServerMessages::PlayerConnected { id } => {
              log::info!("Player {} connected.", id);
              let player_entity = commands
                  .spawn(PbrBundle {
                      mesh: meshes.add(Mesh::from(shape::Cube { size: PLAYER_SIZE })),
                      material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                      transform: Transform::from_translation(PLAYER_SPAWN_POINT),
                      ..Default::default()
                  })
                  .id();

              lobby.players.insert(id, player_entity);
          }
          ServerMessages::PlayerDisconnected { id } => {
              println!("Player {} disconnected.", id);
              if let Some(player_entity) = lobby.players.remove(&id) {
                  commands.entity(player_entity).despawn();
              }
          }
      }
  }
  
  // 
  while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
      let players: HashMap<ClientId, [f32; 3]> = bincode::deserialize(&message).unwrap();
      for (player_id, translation) in players.iter() {
          if let Some(player_entity) = lobby.players.get(player_id) {
              let transform = Transform {
                  translation: (*translation).into(),
                  ..Default::default()
              };
              commands.entity(*player_entity).insert(transform);
          }
      }
  }
}
