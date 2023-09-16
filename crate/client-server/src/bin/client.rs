use bevy::window::WindowResolution;
use bevy::{
  diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
  prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pih_pah::feature::lobby::LobbyDefaultPlugins;
use pih_pah::feature::multiplayer::{
  new_renet_client, panic_on_error_system, Lobby, PlayerInput, ServerMessages, TransportData,
  PLAYER_SIZE, PLAYER_SPAWN_POINT,
};
use pih_pah::feature::music::MusicPlugins;
use pih_pah::feature::ui::{FpsPlugins, UiPlugins};
use pih_pah::lib::netutils::{is_http_address, is_ip_with_port};

use bevy_renet::{
  renet::{DefaultChannel, RenetClient},
  transport::NetcodeClientPlugin,
  RenetClientPlugin,
};

#[cfg(not(any(feature = "wayland", feature = "x11")))]
compile_error!("Either 'wayland' or 'x11' feature must be enabled flag.");

fn main() {
  env_logger::init();
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    println!("Usage: ");
    println!("  client '<ip>:<port>'");
    println!("  client 'example.com'");

    panic!("Not enough arguments.");
  }

  // Checking if the address is either an HTTP address or an IP address with port
  let server_addr = match &args[1] {
    addr if is_http_address(addr) => addr,
    addr if is_ip_with_port(addr) => addr,
    _ => panic!("Invalid argument, must be an HTTP address or an IP with port."),
  };

  let is_debug = std::env::var("DEBUG").is_ok();

  let mut app = App::new();
  app.init_resource::<Lobby>();

  if !is_debug {
    app.add_plugins((DefaultPlugins, EguiPlugin));
  } else {
    let window_plugin_override = WindowPlugin {
      primary_window: Some(Window {
        title: "pih-pah".into(),
        resolution: WindowResolution::default(),
        position: WindowPosition::new(IVec2::new(960, 0)),
        // Tells wasm to resize the window according to the available canvas
        fit_canvas_to_parent: true,
        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
        prevent_default_event_handling: false,
        ..default()
      }),
      ..default()
    };
    app.add_plugins(DefaultPlugins.set(window_plugin_override));
    app.add_plugins(EguiPlugin);
    app.add_plugins(FpsPlugins);
    app.add_plugins(LogDiagnosticsPlugin::default());
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(WorldInspectorPlugin::default());
  }

  app.add_plugins((MusicPlugins, UiPlugins, LobbyDefaultPlugins));
  // some for connection
  app.init_resource::<TransportData>();
  //
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
  player_input.right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
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
  mut transport_data: ResMut<TransportData>,
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

  while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
    transport_data.data = bincode::deserialize(&message).unwrap();
    for (player_id, data) in transport_data.data.iter() {
      if let Some(player_entity) = lobby.players.get(player_id) {
        let transform = Transform {
          translation: (data.0).into(),
          rotation: Quat::from_array(data.1),
          ..Default::default()
        };
        commands.entity(*player_entity).insert(transform);
      }
    }
  }
}
