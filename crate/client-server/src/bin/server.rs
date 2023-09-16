use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowPlugin};
use bevy_egui::EguiPlugin;
use bevy_xpbd_3d::prelude::*;

use pih_pah::feature::lobby::LobbyMinimalPlugins;
use pih_pah::feature::multiplayer::{
  new_renet_server, panic_on_error_system, server_sync_players, server_update_system, Lobby,
  TransportData,
};
use pih_pah::feature::ui::FpsPlugins;
use pih_pah::lib::netutils;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_renet::{renet::RenetServer, transport::NetcodeServerPlugin, RenetServerPlugin};

fn main() {
  std::env::set_var(
    "RUST_LOG",
    std::env::var("RUST_LOG").unwrap_or(String::from("info")),
  );
  env_logger::init();

  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    println!("Usage: ");
    println!("  server '<ip>:<port>'");
    println!("  server 'example.com'");
    panic!("Not enough arguments.");
  }

  // Checking if the address is either an HTTP address or an IP address with port
  let server_addr = match &args[1] {
    addr if netutils::is_http_address(addr) => addr,
    addr if netutils::is_ip_with_port(addr) => addr,
    _ => panic!("Invalid argument, must be an HTTP address or an IP with port."),
  };

  let is_debug = std::env::var("DEBUG").is_ok();

  let mut app = App::new();
  app.init_resource::<Lobby>();

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
    app.add_plugins(FpsPlugins);
    app.add_plugins(LogDiagnosticsPlugin::default());
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(WorldInspectorPlugin::default());
  }

  // Plugins that's always there
  app.add_plugins(LobbyMinimalPlugins);
  app.add_plugins(PhysicsPlugins::default());
  app.add_plugins(RenetServerPlugin);
  app.add_plugins(NetcodeServerPlugin);

  let (server, transport) = new_renet_server(server_addr);
  app.insert_resource(server);
  app.insert_resource(transport);
  //some about connection
  app.init_resource::<TransportData>();

  app.add_systems(
    Update,
    (server_update_system, server_sync_players).run_if(resource_exists::<RenetServer>()),
  );

  app.add_systems(Update, panic_on_error_system);

  app.run();
}
