use bevy::prelude::{shape::Plane, *};
use shared::feature::lobby::PLANE_SIZE;

pub struct ScenePlugins;

impl Plugin for ScenePlugins {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup_scene);
  }
}

fn setup_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // plane
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(Plane::from_size(PLANE_SIZE))),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });
  // light
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 1500.0,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..default()
  });
  // camera
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    camera: Camera {
      order: 2,
      ..default()
    },
    ..default()
  });
}
