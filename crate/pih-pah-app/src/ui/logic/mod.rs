mod menu;
mod options;

pub use menu::*;
pub use options::*;
use bevy::prelude::*;

pub struct UiLogicPlugins;

impl Plugin for UiLogicPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuPlugins, OptionsPlugins));
    }
}