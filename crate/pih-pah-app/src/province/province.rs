use bevy::prelude::*;
use crate::province::menu::MenuPlugins;

pub struct ProvincePlugins;

impl Plugin for ProvincePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(MenuPlugins);
    }
}