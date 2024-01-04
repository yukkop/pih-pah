mod menu;
mod options;

use bevy::prelude::*;
pub use menu::*;
pub use options::*;

pub struct UiLogicPlugins;

impl Plugin for UiLogicPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuPlugins, OptionsPlugins));
    }
}
