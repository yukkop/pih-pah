use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::prelude::PhysicsPlugins;
use pih_pah_app::world::WorldPlugins;

#[cfg(not(any(feature = "wayland", feature = "x11", feature = "windows")))]
compile_error!("Either 'wayland' or 'x11' or 'windows' feature must be enabled flag.");

fn main() {
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or(String::from("info")),
    );
    env_logger::init();
    info!("Starting pih-pah");
    let _args: Vec<String> = std::env::args().collect();

    let is_debug = std::env::var("DEBUG").is_ok();

    let mut app = App::new();

    if !is_debug {
        app.add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "asset".into(),
                ..default()
            }),
            EguiPlugin,
        ));
    } else {
        let window_plugin_override = WindowPlugin {
            primary_window: Some(Window {
                title: "pih-pah".into(),
                resolution: WindowResolution::default(),
                present_mode: PresentMode::AutoNoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        };
        app.add_plugins((
            DefaultPlugins.set(window_plugin_override).set(AssetPlugin {
                file_path: "asset".into(),
                ..default()
            }),
            EguiPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::default());
    }
    info!("Starting pih-pah");

    app.add_plugins(PhysicsPlugins::new(Update));
    app.add_plugins(WorldPlugins);

    app.run();
}
