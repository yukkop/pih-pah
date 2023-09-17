use crate::feature::lobby::PLANE_SIZE;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub struct ScenePlugins;

impl Plugin for ScenePlugins {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup_scene);
  }
}

fn setup_scene(mut commands: Commands) {
  // plane
  commands.spawn(
    // server plane got a collider & rigit body
    (
      PbrBundle {
        ..Default::default()
      },
      RigidBody::Static,
      Collider::cuboid(PLANE_SIZE, 0.002, PLANE_SIZE),
    ),
  );
}
