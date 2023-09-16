use crate::feature::lobby::server::PlayerPlugins;
use crate::feature::lobby::server::ScenePlugins;

use bevy::prelude::*;

// server use minimal setup, without assets, textures etс.
pub struct LobbyPlugins;

impl Plugin for LobbyPlugins {
  fn build(&self, app: &mut App) {
    log::info!("please");
    app.add_plugins(PlayerPlugins).add_plugins(ScenePlugins);
  }
}
