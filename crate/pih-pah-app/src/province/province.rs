use crate::province::menu::MenuPlugins;
use crate::province::ShootingRangePlugins;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ProvinceState {
    #[default]
    Menu = 0,
    ShootingRange = 1,
}

pub struct ProvincePlugins;

impl Plugin for ProvincePlugins {
    fn build(&self, app: &mut App) {
        app.add_state::<ProvinceState>()
            .add_plugins((MenuPlugins, ShootingRangePlugins));
    }
}
