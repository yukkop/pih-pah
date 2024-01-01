use bevy::prelude::*;

use crate::ui::MenuPlugins;

/// Main state of the game
#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    /// Main game menu
    Menu,
    /// Level editor
    Editor,
}

/// Main plugin of the game
pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins(MenuPlugins);
    }
}

/// Open the menu
/// 
/// This function is used from any state to open the menu and 
/// teardown the current state and its resources.
pub fn open_menu(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Menu);
}

/// Open the editor
/// 
/// This function is used from any state to open the editor and
/// teardown the current state and its resources.
pub fn open_editor(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Editor);
}