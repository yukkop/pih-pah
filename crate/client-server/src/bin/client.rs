use bevy::window::WindowResolution;
use bevy::{
  diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
  prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pih_pah::feature::lobby::client::spawn_camera;
use pih_pah::feature::lobby::client::spawn_client_side_player;
use pih_pah::feature::lobby::client::LobbyPlugins;
use pih_pah::feature::multiplayer::{
  new_renet_client, panic_on_error_system, Lobby, PlayerInput, ServerMessages, TransportData,
};
use pih_pah::feature::music::MusicPlugins;
use pih_pah::feature::ui::{FpsPlugins, UiPlugins};
use pih_pah::lib::netutils::{is_http_address, is_ip_with_port};
use renet::ClientId;

use pih_pah::feature::lobby::client::camera_switch;
use pih_pah::feature::multiplayer::{OwnId, player_input, client_send_input, client_sync_players};

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

  app.add_plugins((MusicPlugins, UiPlugins, LobbyPlugins));
  // some for connection
  app.init_resource::<TransportData>();
  //
  app.add_plugins(RenetClientPlugin);
  app.add_plugins(NetcodeClientPlugin);
  app.init_resource::<PlayerInput>();
  app.init_resource::<OwnId>();
  let (client, transport) = new_renet_client(server_addr.to_string());
  app.insert_resource(client);
  app.insert_resource(transport);

  app.add_systems(
    Update,
    (
      player_input,
      camera_switch,
      client_send_input,
      client_sync_players,
    )
      .run_if(bevy_renet::transport::client_connected()),
  );

  app.add_systems(Update, panic_on_error_system);

  app.run();
}
