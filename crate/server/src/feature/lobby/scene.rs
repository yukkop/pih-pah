use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_xpbd_3d::prelude::*;

#[derive(Component, Debug, Clone, InspectorOptions, Reflect /*, Serialize, Deserialize */)]
struct PromiseMesh(Handle<Mesh>);

pub struct ScenePlugins;

impl Plugin for ScenePlugins {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_scene)
      .add_systems(Update, process_colliders);
  }
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
  let mesh_handle = asset_server.load("terrain.gltf#Mesh0/Primitive0");

  commands.spawn((Name::new("GltfMesh"), PromiseMesh(mesh_handle)));

  let mesh_handle = asset_server.load("walking.gltf#Scene0");

  commands.spawn((
    SceneBundle {
      scene: mesh_handle.clone(),
      transform: Transform::from_translation(Vec3::new(0., 4.5, 0.)),
      ..Default::default()
    },
    Name::new("My personajjj"),
  ));

}

fn process_colliders(
  mut commands: Commands,
  meshes: ResMut<Assets<Mesh>>,
  promise_query: Query<(Entity, &PromiseMesh)>,
) {
  for (entity, PromiseMesh(collider_handler)) in promise_query.iter() {
    if let Some(mesh) = meshes.get(collider_handler) {
      let collider = Collider::trimesh_from_mesh(mesh).unwrap();
      commands.entity(entity).insert(RigidBody::Static);
      commands
        .entity(entity)
        .insert(Transform::from_scale(Vec3::splat(16.888))); // 16.88806915283203
      commands.entity(entity).insert(collider);
      commands.entity(entity).remove::<PromiseMesh>();
    }
  }
}
