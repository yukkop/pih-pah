use std::env;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy::winit::WinitWindows;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_xpbd_3d::prelude::PhysicsPlugins;
use pih_pah_app::game::GamePlugins;
use winit::window::Icon;

/// default value for logging 
/// 
/// wgpu_core fluds the logs on info level therefore we need to set it to error
const RUST_LOG_DEFAULT: &str = "info,wgpu_core=error";

/// The directory where the assets are located
const ASSET_DIR: &str = "asset";

/// The path to the icon
const ICON_PATH: &str = "icon-v1.png";

/// The name of the application
const APP_NAME: &str = "pih-pah";

lazy_static::lazy_static! {
    /// The current version of the application
    pub static ref VERSION: String = format!("{}.{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH"));

    /// If the application is running in debug mode
    pub static ref DEBUG: bool = std::env::var("DEBUG").is_ok();

    /// The name of the application with the version
    pub static ref VERSIONED_APP_NAME: String = format!("{APP_NAME} v{}", *VERSION);
}


fn main() {
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or(String::from(RUST_LOG_DEFAULT)),
    );

    let mut app = App::new();

    let asset_plugin = AssetPlugin {
        file_path: ASSET_DIR.into(),
        ..default()
    };

    if !*DEBUG {
        let window_plugin_override = WindowPlugin {
            primary_window: Some(Window {
                title: VERSIONED_APP_NAME.clone(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        };
        app.add_plugins((
            DefaultPlugins.set(window_plugin_override).set(asset_plugin),
            EguiPlugin,
        ));
    } else {
        use bevy_editor_pls::prelude::*;

        let window_plugin_override = WindowPlugin {
            primary_window: Some(Window {
                title: VERSIONED_APP_NAME.clone(),
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
            DefaultPlugins.set(window_plugin_override).set(asset_plugin),
            // editor for easy debugging https://github.com/jakobhellermann/bevy_editor_pls
            // its included egui plugin and egui_inspector plugin
            EditorPlugin::default(),
        ));
    }

    // it can be difficult to make physics undependent from the frame rate
    // but we cannot use FixedUpdate because it is not supported by bevy_xpbd_3d as well as
    app.add_plugins(PhysicsPlugins::new(Update))
        .add_systems(Startup, set_window_icon)
        .add_plugins(GamePlugins);


info!("Starting {APP_NAME} v{}", *VERSION);

    app.run();
}

fn set_window_icon(windows: NonSend<WinitWindows>) {
    let exe_path = env::current_exe().expect("Failed to find executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to find executable directory");
    let (icon_rgba, icon_width, icon_height) = {
        if let Ok(image) = image::open(exe_dir.join(ICON_PATH)) {
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

    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
