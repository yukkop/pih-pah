use std::fmt::Formatter;

use bevy::{prelude::*, utils::HashMap, ecs::system::SystemId};
use strum_macros::EnumIter;

use crate::{ui::MenuPlugins, option::OptionsPlugins, sound::SoundPlugins, asset_loader::{AssetLoaderPlugins, DEFAULT_LEVEL}, util::validate_hash_map, controls::ControlsPlugin, lobby::LobbyPlugins};

/// Main state of the game
#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone, Reflect)]
pub enum GameState {
    #[default]
    CoreLoading,
    /// Main game menu
    Menu,
    /// Prepare level to load
    /// ei symlink current level directory to chosen level directory
    /// in levels directory
    LevelEditorPreload,
    /// Level editor load
    /// load assets from chosen level directory
    LevelEditorLoad,
    /// Level editor
    LevelEditor,
}

/// Global application actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum GlobalAction {
    /// Open the menu via `open_menu` system
    /// 
    /// Used from any `GameState` to open the menu and 
    /// teardown the current `GameState` and its resources.
    OpenMenu,
    /// Open the editor via `open_editor` system
    /// 
    /// Used from any `GameState` to open the editor and
    /// teardown the current `GameState` and its resources.
    OpenLevelEditor,
}

/// Resource that contains all global actions
#[derive(Default, Resource, Deref, DerefMut)]
pub struct GlobalActions(HashMap<GlobalAction, SystemId>);

// TODO: make automatic derive for this
impl GlobalActions {
    pub fn get(&self, action: GlobalAction) -> SystemId {
        self.0.get(&action).copied().unwrap()
    }
}

/// Resource that contains path to level in levels directory
#[derive(Debug, Resource, Deref, DerefMut)]
pub struct CurrentLevelPath(String);

impl CurrentLevelPath {
    pub fn get(&self) -> &str {
        &self.0
    }
}

impl Default for CurrentLevelPath {
    fn default() -> Self {
        Self(DEFAULT_LEVEL.into())
    }
}

impl core::fmt::Display for CurrentLevelPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Main plugin of the game
pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentLevelPath>()
            .init_resource::<GlobalActions>()
            .add_systems(Startup, register)
            .add_state::<GameState>().add_plugins((MenuPlugins, SoundPlugins, AssetLoaderPlugins, LobbyPlugins, ControlsPlugin, OptionsPlugins));
    }
}

/// System that runs once at startup to register all menu actions systems
fn register(
    world: &mut World,
) {
    let open_menu_id = world.register_system(open_menu);
    let open_level_editor_id = world.register_system(open_level_editor);

    if let Some(mut global_actions) = world.get_resource_mut::<GlobalActions>() {
        global_actions.insert(GlobalAction::OpenMenu, open_menu_id);
        global_actions.insert(GlobalAction::OpenLevelEditor, open_level_editor_id);

        // If you see this error, you may add new action in menu_actions
        // or make sure that you have only one MenuAction with the same name in the MenuActions 
        assert!(validate_hash_map(&global_actions));
    }
}


/// Open the menu
/// 
/// This function is used from any `GameState` to open the menu and 
/// teardown the current `GameState` and its resources.
pub fn open_menu(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Menu);
}

/// Open the editor
/// 
/// This function is used from any `GameState` to open the editor and
/// teardown the current `GameState` and its resources.
pub fn open_level_editor(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::LevelEditorPreload);
}