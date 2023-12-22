use std::{
    env,
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Arc, error,
};

use bevy::{
    app::{App, Last, Plugin, PostStartup},
    asset::Assets,
    ecs::{
        event::{Event, EventReader},
        system::{Commands, Res, ResMut, Resource},
    },
    log::warn,
    prelude::{Deref, DerefMut},
};
use bevy_kira_audio::{prelude::Volume, AudioInstance, AudioTween};
use serde::{self, Deserialize, Serialize};

use crate::sound::MenuMusic;

#[allow(dead_code)]
#[derive(Debug, Resource, Default, Deref, DerefMut)]
struct AppliedSettings(Settings);

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Resource)]
pub struct Settings {
    pub music_volume: f64,
    pub sensativity: f32,
    pub effect_volume: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self { music_volume: 10., effect_volume: 10., sensativity: 0.5 }
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
            sensativity: applied_settings.sensativity,
            effect_volume: applied_settings.effect_volume,
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

        commands.insert_resource(AppliedSettings(Settings {
            music_volume: settings.music_volume,
            sensativity: settings.sensativity,
            effect_volume: settings.effect_volume,
        }));

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

    fn to_corrupted(err: impl error::Error, path: &PathBuf) {
        log::warn!("Failed to open settings file: {:#?}, it will be renamed into {:?}", err, path.to_str());
        let new_path = path.with_extension("corrupted");
        std::fs::rename(&path, &new_path).expect("Failed to rename corrupted file");
    }

    fn create_new_settings_file(path: &PathBuf, commands: &mut Commands) -> Settings {
        log::warn!("Creating new settings file: {:#?} with default values", path);
        let mut file = File::create(path).expect("Failed to create settings file");
        commands.insert_resource(SettingsPath(Arc::new(path.clone())));

        let settings = Settings::default();
        serde_yaml::to_writer(&mut file, &settings).expect("Failed to write to settings file");

        settings
    }

    let settings = {
        if yaml_path.exists() {
            let file = File::open(&yaml_path);
    
            match file {
                Ok(file) => {
                    commands.insert_resource(SettingsPath(yaml_path.clone().into()));
    
                    match serde_yaml::from_reader(file) {
                        Ok(settings) => settings,
                        Err(err) => {
                            to_corrupted(err, &yaml_path);
                            create_new_settings_file(&yaml_path, &mut commands)
                        }
                    }
                }
                Err(err) => {
                    to_corrupted(err, &yaml_path);
                    create_new_settings_file(&yaml_path, &mut commands)
                }
            }
        } else if yml_path.exists() {
            let file = File::open(&yaml_path);
    
            match file {
                Ok(file) => {
                    commands.insert_resource(SettingsPath(yml_path.clone().into()));
    
                    match serde_yaml::from_reader(file) {
                        Ok(settings) => settings,
                        Err(err) => {
                            to_corrupted(err, &yml_path);
                            create_new_settings_file(&yml_path, &mut commands)
                        }
                    }
                }
                Err(err) => {
                    to_corrupted(err, &yml_path);
                    create_new_settings_file(&yml_path, &mut commands)
                }
            } 
        } else {
            create_new_settings_file(&yaml_path, &mut commands)
        }
    };

    commands.insert_resource(settings);
}
