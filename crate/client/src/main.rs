use std::sync::Arc;

use bevy::window::WindowResolution;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::EguiPlugin;
use shared::feature::multiplayer::panic_on_error_system;
use client::{
  feature::{
    lobby::LobbyPlugins,
    multiplayer::MultiplayerPlugins,
    music::MusicPlugins,
    ui::{UiDebugPlugins, UiPlugins},
  },
};
use shared::lib::netutils::{is_http_address, is_ip_with_port};

#[cfg(not(any(feature = "wayland", feature = "x11", feature = "windows")))]
compile_error!("Either 'wayland' or 'x11' feature must be enabled flag.");

fn main() {
  env_logger::init();
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    println!("Usage: ");
    println!("  multiplayer '<ip>:<port>'");
    println!("  multiplayer 'example.com'");

    panic!("Not enough arguments.");
  }

  // Checking if the address is either an HTTP address or an IP address with port
  let api_url = match &args[1] {
    addr if is_http_address(addr) => addr,
    addr if is_ip_with_port(addr) => addr,
    _ => panic!("Invalid argument, must be an HTTP address or an IP with port."),
  };

  let is_debug = std::env::var("DEBUG").is_ok();

  let mut app = App::new();

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
    app.add_plugins(UiDebugPlugins);
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    // app.add_plugins(LogDiagnosticsPlugin::default());
    // app.add_plugins(WorldInspectorPlugin::default());
  }

  app.add_plugins((
    MusicPlugins,
    UiPlugins::by_string(Arc::new(api_url.to_string())),
    LobbyPlugins,
    MultiplayerPlugins,
  ));

  app.add_systems(Update, panic_on_error_system);

  app.run();
}
