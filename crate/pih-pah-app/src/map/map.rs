use std::fmt::Display;

use crate::map::menu::MenuPlugins;
use crate::map::ShootingRangePlugins;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::spawn_point::SpawnPoint;
use super::GravityHellPlugins;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Serialize, Deserialize)]
pub enum MapState {
    #[default]
    Menu = 0,
    ShootingRange = 1,
    GravityHell = 2,
}

impl Display for MapState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapState::Menu => write!(f, "Menu"),
            MapState::ShootingRange => write!(f, "ShootingRange"),
            MapState::GravityHell => write!(f, "GravityHell"),
        }
    }
}

pub fn is_loaded(spawn_point: &Res<SpawnPoint>) -> bool {
    !spawn_point.is_empty()
}

pub struct MapPlugins;

impl Plugin for MapPlugins {
    fn build(&self, app: &mut App) {
        app.add_state::<MapState>()
            .init_resource::<SpawnPoint>()
            .add_plugins((MenuPlugins, ShootingRangePlugins, GravityHellPlugins));
    }
}
