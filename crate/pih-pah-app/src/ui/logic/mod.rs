mod menu;
pub use menu::*;
use bevy::prelude::*;

pub struct UiLogicPlugins;

impl Plugin for UiLogicPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(MenuPlugins);
    }
}