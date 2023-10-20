use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowPlugin};
use bevy_egui::EguiPlugin;
use bevy_xpbd_3d::prelude::*;

use server::feature::lobby::LobbyPlugins;
use server::feature::multiplayer::MultiplayerPlugins;
// use server::feature::UiDebugPlugins;

use server::feature::heartbeat::HeartbeatPlugins;
use shared::feature::multiplayer::panic_on_error_system;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
  std::env::set_var(
    "RUST_LOG",
    std::env::var("RUST_LOG").unwrap_or(String::from("info")),
  );
  env_logger::init();

  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    println!("Usage: ");
    println!("  server '<server address>' '<load-balanser address>'");
    panic!("Not enough arguments.");
  }

  // to listen clients
  let listen_addr = &args[1];
  // to send online reports to main server
  let send_addr = &args[2];

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
    app.add_plugins(MinimalPlugins);
  } else {
    // Debug plugins
    app.add_plugins(DefaultPlugins.set(window_plugin_override));
    app.add_plugins(EguiPlugin);
    // app.add_plugins(UiDebugPlugins);
    app.add_plugins(LogDiagnosticsPlugin::default());
    // app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(WorldInspectorPlugin::default());
  }

  // Plugins that's always there
  app.add_plugins(LobbyPlugins);
  app.add_plugins(PhysicsPlugins::default());
  app.add_plugins(MultiplayerPlugins::by_string(listen_addr.to_string()));
  app.add_plugins(HeartbeatPlugins::by_string(
    send_addr.to_string(),
    listen_addr.to_string(),
  ));

  app.add_systems(Update, panic_on_error_system);

  app.run();
}