use bevy::prelude::*;

pub struct ScenePlugins;

impl Plugin for ScenePlugins {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, (setup_scene, spawn_gltf_mesh));
  }
}

fn setup_scene(
  mut commands: Commands,
  ass: Res<AssetServer>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mesh_handle = ass.load("terrain.gltf#Mesh0/Primitive0");

  commands.spawn((
    PbrBundle {
      mesh: mesh_handle.clone(),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      transform: Transform::from_scale(Vec3::splat(16.888)), // 16.88806915283203
      ..Default::default()
    },
    Name::new("GltfMesh"),
  ));

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

fn spawn_gltf_mesh() {}
