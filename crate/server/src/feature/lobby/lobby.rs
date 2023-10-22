use crate::feature::lobby::PlayerPlugins;
use crate::feature::lobby::ScenePlugins;

use bevy::prelude::*;

// server use minimal setup, without assets, textures etс.
pub struct LobbyPlugins;

impl Plugin for LobbyPlugins {
  fn build(&self, app: &mut App) {
    app.add_plugins(PlayerPlugins).add_plugins(ScenePlugins);
  }
}
