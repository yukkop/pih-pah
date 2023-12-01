use bevy::prelude::*;
use crate::province::menu::MenuPlugins;
use crate::province::ShootingRangePlugins;

// TODO implementation instead of load & unload actions
enum ProvinceState {
    Menu,
    ShootingRange
}

pub struct ProvincePlugins;

impl Plugin for ProvincePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuPlugins, ShootingRangePlugins));
    }
}