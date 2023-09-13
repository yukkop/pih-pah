pub mod hud;
use self::hud::HudPlugins;

use bevy::prelude::*;

pub struct UiPlugins;

impl Plugin for UiPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(HudPlugins);
    }
}
