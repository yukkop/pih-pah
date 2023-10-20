use bevy::prelude::*;

use super::scene::ScenePlugins;

pub struct LobbyPlugins;

impl Plugin for LobbyPlugins {
  fn build(&self, app: &mut App) {
    app.add_plugins(ScenePlugins);
  }
}
