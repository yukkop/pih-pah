use std::{env, fs::File};

use bevy::{
    app::{App, Plugin, PreStartup},
    ecs::system::{Commands, Resource},
};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Resource)]
pub struct Settings {
    music_volume: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { music_volume: 0.5 }
    }
}

pub struct SettingsPlugins;

impl Plugin for SettingsPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

fn setup(mut commands: Commands) {
    let exe_path = env::current_exe().expect("Failed to find executable path");

    let exe_dir = exe_path
        .parent()
        .expect("Failed to find executable directory");

    let yaml_path = exe_dir.join("settings.yaml");
    let yml_path = exe_dir.join("settings.yml");

    let settings = {
        if yaml_path.exists() {
            let file = File::open(&yaml_path).unwrap_or_else(|_| {
                panic!("Failed to open exist settings file ({:#?})", &yaml_path)
            });
            serde_yaml::from_reader(file)
                .unwrap_or_else(|_| panic!("Failed to read settings file ({:#?})", &yaml_path))
        } else if yml_path.exists() {
            let file = File::open(&yml_path).unwrap_or_else(|_| {
                panic!("Failed to open exist settings file ({:#?})", &yml_path)
            });
            serde_yaml::from_reader(&file)
                .unwrap_or_else(|_| panic!("Failed to read settings file ({:#?})", &yml_path))
        } else {
            let mut file: File = File::create(&yaml_path)
                .unwrap_or_else(|_| panic!("Failed to create settings file ({:#?})", &yaml_path));

            let settings = Settings::default();
            serde_yaml::to_writer(&mut file, &settings)
                .unwrap_or_else(|_| panic!("Failed to write to settings file ({:#?})", &yaml_path));

            settings
        }
    };

    commands.insert_resource(settings);
}
