use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use shared::feature::lobby::PLANE_SIZE;

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
      Friction::new(0.4),
      RigidBody::Static,
      Collider::cuboid(PLANE_SIZE, 0.002, PLANE_SIZE),
      // NERV HUESOS
    ),
  );
}
