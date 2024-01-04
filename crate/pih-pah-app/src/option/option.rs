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
    prelude::{Deref, DerefMut},
};
use bevy_kira_audio::{prelude::Volume, AudioInstance, AudioTween};
use serde::{self, Deserialize, Serialize};

use crate::sound::MusicResource;

/// Path to the options if options file yaml
const OPTIONS_PATH_YAML: &str = "options.yaml";
/// Path to the options if options file yml
const OPTIONS_PATH_YML: &str = "options.yml";

/// Options that already applied
#[allow(dead_code)]
#[derive(Debug, Resource, Default, Deref, DerefMut)]
struct AppliedOptions {
    music_volume: f64,
}

/// Options that will be applied or have option to be exempted
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Resource)]
pub struct Options {
    pub music_volume: f64,
}

impl Default for Options {
    fn default() -> Self {
        Self { music_volume: 10. }
    }
}

#[derive(Debug, Resource, Deref)]
struct OptionsPath(Arc<PathBuf>);

/// Event that applies options from `Options` to the all game
#[derive(Debug, Event)]
pub struct ApplyOptions;

/// Event that exempts `Options` to last applied options
#[derive(Debug, Event)]
pub struct ExemptOptions;

/// Plugin that responsible for game options
///
/// SAFETY: `OptionsPlugin` depends to many plugins namely to plugins resourses
/// therefore it should be added to `App` after them
pub struct OptionsPlugins;

impl Plugin for OptionsPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppliedOptions>()
            .add_event::<ApplyOptions>()
            .add_event::<ExemptOptions>()
            .add_systems(PostStartup, setup)
            .add_systems(Last, (apply_options, exempt_options));
    }
}

/// Exempt `Options` to last applied options
fn exempt_options(
    mut commands: Commands,
    mut event: EventReader<ExemptOptions>,
    applied_options: Res<AppliedOptions>,
) {
    for _ in event.read() {
        commands.insert_resource(Options {
            music_volume: applied_options.music_volume,
        });
    }
}

/// System that applies all options from `Options` to the all game
fn apply_options(
    mut event: EventReader<ApplyOptions>,
    options: Res<Options>,
    menu_music: ResMut<MusicResource>,
    mut audio_sources: ResMut<Assets<AudioInstance>>,
    options_path: Res<OptionsPath>,
    mut applied_options: ResMut<AppliedOptions>,
) {
    for _ in event.read() {
        // Apply options to the music
        if let Some(instance) = audio_sources.get_mut(&menu_music.instance_handle) {
            instance.set_volume(
                Volume::Amplitude(options.music_volume / 10.),
                AudioTween::default(),
            );

            // Update applied applied options
            applied_options.music_volume = options.music_volume;
        } else {
            warn!("Failed to get music source");
        }

        // Save options to the file
        let options_path = options_path.as_ref().as_ref();
        let mut file = OpenOptions::new()
            .write(true)
            .open(options_path)
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to create options file ({:#?}) \n error: {:#?}",
                    options_path, err
                )
            });
        file.write_all(serde_yaml::to_string(options.as_ref()).unwrap().as_bytes())
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to write to options file ({:#?}) \n error: {:#?}",
                    options_path, err
                )
            });
    }
}

/// System that setup options
///
/// It reads options from the file or creates new file with default options
/// if file exists but it is not valid yaml file, corrupted rename it to `options.yaml.corrupted`
/// and create new file with default options
fn setup(mut commands: Commands) {
    let exe_path = env::current_exe().expect("Failed to find executable path");

    let exe_dir = exe_path
        .parent()
        .expect("Failed to find executable directory");

    let yaml_path = exe_dir.join(OPTIONS_PATH_YAML);
    let yml_path = exe_dir.join(OPTIONS_PATH_YML);

    let options = {
        if yaml_path.exists() {
            let file = File::open(&yaml_path).unwrap_or_else(|err| {
                panic!(
                    "Failed to open exist options file ({:#?}) \n error: {:#?}",
                    &yaml_path, err
                )
            });

            commands.insert_resource(OptionsPath(yaml_path.clone().into()));

            serde_yaml::from_reader(file).unwrap_or_else(|err| {
                panic!(
                    "Failed to read options file ({:#?}) \n error: {:#?}",
                    &yaml_path, err
                )
            })
        } else if yml_path.exists() {
            let file = File::open(&yml_path).unwrap_or_else(|err| {
                panic!(
                    "Failed to open exist options file ({:#?}) \n error: {:#?}",
                    &yml_path, err
                )
            });

            commands.insert_resource(OptionsPath(yml_path.clone().into()));

            serde_yaml::from_reader(&file).unwrap_or_else(|err| {
                panic!(
                    "Failed to read options file ({:#?}) \n error: {:#?}",
                    &yml_path, err
                )
            })
        } else {
            let mut file: File = File::create(&yaml_path).unwrap_or_else(|err| {
                panic!(
                    "Failed to create options file ({:#?}) \n error: {:#?}",
                    &yaml_path, err
                )
            });

            commands.insert_resource(OptionsPath(yaml_path.clone().into()));

            let options = Options::default();
            serde_yaml::to_writer(&mut file, &options).unwrap_or_else(|err| {
                panic!(
                    "Failed to write to options file ({:#?}) \n error: {:#?}",
                    &yaml_path, err
                )
            });

            options
        }
    };

    commands.insert_resource(options);
}
