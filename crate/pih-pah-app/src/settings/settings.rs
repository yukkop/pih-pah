use std::{env, fs::File, io::Write, io::Read};

use bevy::{app::{Plugin, App, Startup, PreStartup}, ecs::{schedule::OnEnter, system::{Resource, Commands}}, transform::commands};
use bincode::de;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Resource)]
pub struct Settings {
    music_Volume: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
           music_Volume: 0.5,
        }
    }
    
}

pub struct SettingsPlugins;

impl Plugin for SettingsPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup,
                        setup);
    }
}

fn setup(
    mut commands: Commands,
) {
    let exe_path = env::current_exe().expect("Failed to find executable path");
    
    let exe_dir = exe_path.parent().expect("Failed to find executable directory");

    let yaml_path = exe_dir.join("settings.yaml");
    let yml_path = exe_dir.join("settings.yml");

    let settings = {
        if yaml_path.exists() {
           let file = File::open(&yaml_path).expect(format!("Failed to open exist settings file ({:#?})", &yaml_path).as_str());
           serde_yaml::from_reader(file).expect(format!("Failed to read settings file ({:#?})", &yaml_path).as_str())
        } else if yml_path.exists() {
           let file = File::open(&yml_path).expect(format!("Failed to open exist settings file ({:#?})", &yml_path).as_str());
           serde_yaml::from_reader(&file).expect(format!("Failed to read settings file ({:#?})", &yml_path).as_str())
        } else {
            let mut file: File = File::create(&yaml_path).expect(format!("Failed to create settings file ({:#?})", &yaml_path).as_str());

            let settings = Settings::default();
            serde_yaml::to_writer(&mut file, &settings).expect(format!("Failed to write to settings file ({:#?})", &yaml_path).as_str());

            settings
        }
    };

    commands.insert_resource(settings);
}