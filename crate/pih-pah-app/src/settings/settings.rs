use std::{
    env,
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Arc,
};

use bevy::{
    app::{App, Last, Plugin, PostStartup},
    asset::Assets,
    ecs::{
        event::{Event, EventReader},
        system::{Commands, Res, ResMut, Resource},
    },
    log::warn,
    prelude::Deref,
};
use bevy_kira_audio::{prelude::Volume, AudioInstance, AudioTween};
use serde::{self, Deserialize, Serialize};

use crate::sound::MenuMusic;

#[allow(dead_code)]
#[derive(Debug, Resource, Default)]
struct AppliedSettings {
    music_volume: f64,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Resource)]
pub struct Settings {
    pub music_volume: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self { music_volume: 10. }
    }
}

#[derive(Debug, Resource, Deref)]
struct SettingsPath(Arc<PathBuf>);

#[derive(Debug, Event)]
pub struct ApplySettings;

#[derive(Debug, Event)]
pub struct ExemptSettings;

pub struct SettingsPlugins;

impl Plugin for SettingsPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppliedSettings>()
            .add_event::<ApplySettings>()
            .add_event::<ExemptSettings>()
            .add_systems(PostStartup, setup)
            .add_systems(Last, (apply_settings, exempt_settings));
    }
}

fn exempt_settings(
    mut commands: Commands,
    mut event: EventReader<ExemptSettings>,
    applied_settings: Res<AppliedSettings>,
) {
    for _ in event.read() {
        commands.insert_resource(Settings {
            music_volume: applied_settings.music_volume,
        });
    }
}

fn apply_settings(
    mut commands: Commands,
    mut event: EventReader<ApplySettings>,
    settings: Res<Settings>,
    menu_music: ResMut<MenuMusic>,
    mut audio_sources: ResMut<Assets<AudioInstance>>,
    settings_path: Res<SettingsPath>,
) {
    for _ in event.read() {
        if let Some(instance) = audio_sources.get_mut(&menu_music.instance_handle) {
            instance.set_volume(
                Volume::Amplitude(settings.music_volume / 10.),
                AudioTween::default(),
            );
        } else {
            warn!("Failed to get music source");
        }

        commands.insert_resource(AppliedSettings {
            music_volume: settings.music_volume,
        });

        let settings_path = settings_path.as_ref().as_ref();
        let mut file = OpenOptions::new()
            .write(true)
            .open(settings_path)
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to create settings file ({:#?}) \n error: {:#?}",
                    settings_path, err
                )
            });
        file.write_all(serde_yaml::to_string(settings.as_ref()).unwrap().as_bytes())
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to write to settings file ({:#?}) \n error: {:#?}",
                    settings_path, err
                )
            });
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
            let file = File::open(&yaml_path).unwrap_or_else(|err| {
                panic!(
                    "Failed to open exist settings file ({:#?}) \n error: {:#?}",
                    &yaml_path, err
                )
            });

            commands.insert_resource(SettingsPath(yaml_path.clone().into()));

            serde_yaml::from_reader(file).unwrap_or_else(|err| {
                panic!(
                    "Failed to read settings file ({:#?}) \n error: {:#?}",
                    &yaml_path, err
                )
            })
        } else if yml_path.exists() {
            let file = File::open(&yml_path).unwrap_or_else(|err| {
                panic!(
                    "Failed to open exist settings file ({:#?}) \n error: {:#?}",
                    &yml_path, err
                )
            });

            commands.insert_resource(SettingsPath(yml_path.clone().into()));

            serde_yaml::from_reader(&file).unwrap_or_else(|err| {
                panic!(
                    "Failed to read settings file ({:#?}) \n error: {:#?}",
                    &yml_path, err
                )
            })
        } else {
            let mut file: File = File::create(&yaml_path).unwrap_or_else(|err| {
                panic!(
                    "Failed to create settings file ({:#?}) \n error: {:#?}",
                    &yaml_path, err
                )
            });

            commands.insert_resource(SettingsPath(yaml_path.clone().into()));

            let settings = Settings::default();
            serde_yaml::to_writer(&mut file, &settings).unwrap_or_else(|err| {
                panic!(
                    "Failed to write to settings file ({:#?}) \n error: {:#?}",
                    &yaml_path, err
                )
            });

            settings
        }
    };

    commands.insert_resource(settings);
}
