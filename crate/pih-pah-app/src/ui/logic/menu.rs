use bevy::{prelude::*, ecs::system::SystemId, utils::HashMap, app::AppExit};
use strum_macros::EnumIter;

use crate::{game::GameState, util::validate_hash_map};

/// Main menu state
#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone)]
pub enum MenuWindow {
    #[default]
    /// Main menu without any window
    Empty,
    /// Options window opened from main menu
    Options,
}

/// Menu logic actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum MenuAction {
    /// Key for `open_level_editor` system
    /// that changes game state to `GameState::Editor`
    /// to open editor game mode
    StartLevelEditing,
    /// Key for `exit_from_game` system
    /// that closes application
    Exit,
    OpenOptions,
}

/// Resource that contains all menu logic actions systems
#[derive(Default, Resource, Deref, DerefMut)]
pub struct MenuActions(HashMap<MenuAction, SystemId>);

impl MenuActions {
    pub fn get(&self, action: MenuAction) -> SystemId {
        self.0.get(&action).copied().unwrap()
    }
}

/// Plugin that registers all menu logic actions systems
/// that you may use in menu view layer
pub struct MenuPlugins;

impl Plugin for MenuPlugins {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<MenuActions>()
        .add_state::<MenuWindow>()
        .add_systems(Startup, register);
    }
}

/// System that runs once at startup to register all menu actions systems
fn register(
    world: &mut World,
) {
    let open_level_editor_id = world.register_system(open_level_editor);
    let exit_from_game_id = world.register_system(exit_from_game);
    let open_options_window_id = world.register_system(open_options_window);

    if let Some(mut menu_actions) = world.get_resource_mut::<MenuActions>() {
        menu_actions.insert(MenuAction::StartLevelEditing, open_level_editor_id);
        menu_actions.insert(MenuAction::Exit, exit_from_game_id);
        menu_actions.insert(MenuAction::OpenOptions, open_options_window_id);

        // If you see this error, you may add new action in menu_actions
        // or make sure that you have only one MenuAction with the same name in the MenuActions 
        assert!(validate_hash_map(&menu_actions));
    }
}

/// Change game state to `GameState::Editor`
fn open_level_editor(
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_game_state.set(GameState::LevelEditor);
}

/// Open options window
fn open_options_window(
    mut next_window: ResMut<NextState<MenuWindow>>,
) {
    next_window.set(MenuWindow::Options);
}

/// Close application
fn exit_from_game(
    mut exit: EventWriter<AppExit>,
) {
    exit.send(AppExit);
}