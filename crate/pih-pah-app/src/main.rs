use std::env;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy::winit::WinitWindows;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::prelude::PhysicsPlugins;
use pih_pah_app::world::WorldPlugins;
use winit::window::Icon;

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
    app.add_systems(Startup, set_window_icon);
    app.add_plugins(WorldPlugins);

    app.run();
}

fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    let exe_path = env::current_exe().expect("Failed to find executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to find executable directory");
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        if let Ok(image) = image::open(exe_dir.join("icon-v1.png")) {
            let image = image.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        } else {
            warn!("Failed to load icon");
            return;
        }
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
