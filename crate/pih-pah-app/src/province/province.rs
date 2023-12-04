use std::fmt::Display;

use crate::province::menu::MenuPlugins;
use crate::province::ShootingRangePlugins;
use bevy::prelude::*;

use super::spawn_point::SpawnPoint;
use super::GravityHellPlugins;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ProvinceState {
    #[default]
    Menu = 0,
    ShootingRange = 1,
    GravityHell = 2,
}

impl Display for ProvinceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProvinceState::Menu => write!(f, "Menu"),
            ProvinceState::ShootingRange => write!(f, "ShootingRange"),
            ProvinceState::GravityHell => write!(f, "GravityHell"),
        }
    }
}

pub struct ProvincePlugins;

impl Plugin for ProvincePlugins {
    fn build(&self, app: &mut App) {
        app.add_state::<ProvinceState>()
            .init_resource::<SpawnPoint>()
            .add_plugins((MenuPlugins, ShootingRangePlugins, GravityHellPlugins));
    }
}
