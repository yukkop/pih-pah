use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use std::{path::{Path, PathBuf}, env}; 

use crate::{game::{GameState, CurrentLevelPath}, ASSET_DIR};

/// The name of the custom asset file for the core assets
const CORE_ASSET: &str = "core.assets.ron";

/// The name of the custom asset file for the current level
const CURRENT_LEVEL_ASSET: &str = "current_level.assets.ron";

/// The path to the current level directory
/// that is symlinked to the chosen level directory
/// in `LEVEL_DIR`
const CURRENT_LEVEL_PATH: &str = "current_level";

/// The directory where the levels are located
const LEVEL_DIR: &str = "level";

/// The default level name
pub const DEFAULT_LEVEL: &str = "default";

/// 
#[derive(AssetCollection, Resource)]
pub struct CoreAsset {}

/// Assets for the current level, that is reloade 
/// when the level is changed, or loaded
#[derive(AssetCollection, Resource)]
pub struct LevelAsset  {}

pub struct AssetLoaderPlugins;

impl Plugin for AssetLoaderPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::LevelEditorPreload), link_level)
            .add_loading_state(
                LoadingState::new(GameState::CoreLoading).continue_to_state(GameState::Menu),
            )
            .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
                GameState::CoreLoading,
                CORE_ASSET,
            )
            .add_collection_to_loading_state::<_, CoreAsset>(GameState::CoreLoading)
            .add_loading_state(
                LoadingState::new(GameState::LevelEditorLoad).continue_to_state(GameState::LevelEditor),
            )
            .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
                GameState::LevelEditorLoad,
                CURRENT_LEVEL_ASSET,
            )
            .add_collection_to_loading_state::<_, LevelAsset>(GameState::LevelEditorLoad);
    }
}

/// Link current level directory to chosen level directory
/// using `CurrentLevelPath` to get the path to chosen level directory
fn link_level(
    current_level_path: Res<CurrentLevelPath>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    // TODO: you really want execute `root_asset_path` every time?
    let asset_root = root_asset_path(ASSET_DIR);
    let link = asset_root.join(format!("{CURRENT_LEVEL_PATH}"));
    let original = asset_root.join(LEVEL_DIR).join(current_level_path.get());

    // TODO if link exists, but it is file
    #[cfg(target_os = "windows")]
    {
        if let Err(err) = std::process::Command::new("cmd")
            .arg("/C")
            .arg("rmdir")
            .arg("/S")
            .arg("/Q")
            .arg(&link)
            .output()
        {
            log::error!("Failed to remove directory: {:?}", err)
        }
        if let Err(err) = std::process::Command::new("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(link)
            .arg(original)
            .output()
        {
            // TODO: if symlink already exists, this is not an error
            // TODO: if level directory does not exist, this is an error
            // but we need open at least default level
            // TODO: if default level does not exist, this is an error
            // but we need download it from server 
            log::error!("Failed to create symlink: {err}")
        }

    }

    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::symlink; 

        if !original.exists() {
            match fs::create_dir_all(&original) {
                Ok(_) => println!("Directory created successfully"),
                Err(e) => println!("Error creating directory: {}", e),
            }
        } else {
            println!("Directory already exists");
        }


        if let Err(err) = symlink(
            &original,
            &link,
        ) {
            log::trace!("original: {:?}, link: {:?}", original, link);

            // TODO: if symlink already exists, this is not an error
            // TODO: if level directory does not exist, this is an error
            // but we need open at least default level
            // TODO: if default level does not exist, this is an error
            // but we need download it from server 
            log::error!("Failed to create symlink: {err}");
        }
    }

    next_game_state.set(GameState::LevelEditorLoad);
}

fn root_asset_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let root_path = get_base_path().join(path.as_ref());
    if let Err(e) = std::fs::create_dir_all(&root_path) {
        warn!(
            "Failed to create root directory {:?} for file asset reader: {:?}",
            root_path, e
        );
    }
    root_path
}

fn get_base_path() -> PathBuf {
    if let Ok(manifest_dir) = env::var("BEVY_ASSET_ROOT") {
        PathBuf::from(manifest_dir)
    } else if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir)
    } else {
        env::current_exe()
            .map(|path| {
                path.parent()
                    .map(|exe_parent_path| exe_parent_path.to_owned())
                    .unwrap()
            })
            .unwrap()
    }
}