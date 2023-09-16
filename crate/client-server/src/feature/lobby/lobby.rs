use crate::feature::lobby::player::PlayerPlugins;
use crate::feature::lobby::scene::{SceneDefaultPlugins, SceneMinimalPlugins};

use bevy::prelude::*;

// server use minimal setup, without assets, textures etс.
pub struct LobbyMinimalPlugins;

impl Plugin for LobbyMinimalPlugins {
  fn build(&self, app: &mut App) {
    log::info!("please");
    app.add_plugins((PlayerPlugins, SceneMinimalPlugins));
  }
}

pub struct LobbyDefaultPlugins;

impl Plugin for LobbyDefaultPlugins {
  fn build(&self, app: &mut App) {
    app.add_plugins(SceneDefaultPlugins);
  }
}
