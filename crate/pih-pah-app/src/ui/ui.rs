use bevy::prelude::*;
use crate::ui::menu::MenuPlugins;

pub struct UiPlugins;

impl Plugin for UiPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MenuPlugins)
            .add_systems(Startup, setup);
    }
}

fn setup() {
}
