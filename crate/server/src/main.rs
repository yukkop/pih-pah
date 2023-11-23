use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::render::mesh::MeshPlugin;
use bevy::scene::ScenePlugin;
use bevy::window::{PresentMode, Window, WindowPlugin};
use bevy_egui::EguiPlugin;
use bevy_xpbd_3d::prelude::*;

use server::feature::lobby::LobbyPlugins;
use server::feature::multiplayer::MultiplayerPlugins;

use server::feature::heartbeat::HeartbeatPlugins;
use shared::feature::multiplayer::panic_on_error_system;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shared::lib::netutils::{is_http_address, is_ip_with_port};

fn main() {
  std::env::set_var(
    "RUST_LOG",
    std::env::var("RUST_LOG").unwrap_or(String::from("info")),
  );
  env_logger::init();

  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 || &args[1] == "-h" || &args[1] == "--help" {
    println!("Usage: ");
    println!("  server '<server address>' '<load-balancer address>'");
    panic!("Not enough arguments.");
  }

  // to listen clients
  let listen_addr = match &args[1] {
    addr if is_http_address(addr) => addr,
    addr if is_ip_with_port(addr) => addr,
    _ => panic!("Invalid argument, must be an HTTP address or an IP with port."),
  };

  let is_debug = std::env::var("DEBUG").is_ok();

  let mut app = App::new();

  let window_plugin_override = WindowPlugin {
    primary_window: Some(Window {
      title: "pih-pah".to_string(),
      // this is needed for stable fps
      present_mode: PresentMode::AutoNoVsync,
      ..default()
    }),
    ..default()
  };

  if !is_debug {
    // Normal plugins
    app
      .add_plugins(MinimalPlugins)
      .add_plugins(AssetPlugin::default())
      .add_plugins(MeshPlugin)
      .add_plugins(ScenePlugin)
      .add_plugins(bevy_minimal_gltf::MinimalGltfPlugin::default());
  } else {
    // Debug plugins
    app
      .add_plugins(DefaultPlugins.set(window_plugin_override))
      .add_plugins(EguiPlugin)
      //.add_plugins(UiDebugPlugins);
      .add_plugins(LogDiagnosticsPlugin::default())
      //.add_plugins(FrameTimeDiagnosticsPlugin);
      .add_plugins(WorldInspectorPlugin::default());
  }

  if args.len() >= 3 {
    let addr = match &args[2] {
      addr if is_http_address(addr) => addr,
      addr if is_ip_with_port(addr) => addr,
      _ => panic!("Invalid argument, must be an HTTP address or an IP with port."),
    };

    // to send online reports to main server
    app.add_plugins(HeartbeatPlugins::by_string(
      addr.clone().to_string(),
      listen_addr.to_string(),
    ));
  }

  // Plugins that's always there
  app
    .add_plugins(LobbyPlugins)
    .add_plugins(PhysicsPlugins::default())
    .add_plugins(MultiplayerPlugins::by_string(listen_addr.to_string()))
    .add_systems(Update, panic_on_error_system)
    .run();
}
