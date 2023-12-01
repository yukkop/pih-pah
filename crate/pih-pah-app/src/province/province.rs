use bevy::prelude::*;
use crate::province::menu::MenuPlugins;
use crate::province::ShootingRangePlugins;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ProvinceState {
    #[default]
    Menu,
    ShootingRange
}

pub struct ProvincePlugins;

impl Plugin for ProvincePlugins {
    fn build(&self, app: &mut App) {
        app.add_state::<ProvinceState>().add_plugins((MenuPlugins, ShootingRangePlugins));
    }
}