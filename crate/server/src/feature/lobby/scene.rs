use std::env;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use shared::feature::lobby::PLANE_SIZE;
use bevy_inspector_egui::InspectorOptions;
use bevy_xpbd_3d::math::Vector;

#[derive(Component, Debug, Clone, InspectorOptions, Reflect/*, Serialize, Deserialize */)]
struct PromiseMesh(Handle<Mesh>);

pub struct ScenePlugins;

impl Plugin for ScenePlugins {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_scene)
      .add_systems(Update, process_colliders);
  }
}

fn setup_scene(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  // plane
  commands.spawn(
    // server plane got a collider & rigit body
    (
      Friction::new(0.4),
      RigidBody::Static,
      Collider::cuboid(PLANE_SIZE, 0.002, PLANE_SIZE),
    ),
  );

  let mesh_handle = asset_server.load("terrain-2.glb#Mesh0/Primitive0");

  commands.spawn((
    Name::new("GltfMesh"),
    PromiseMesh(mesh_handle)
  ));
}

fn process_colliders(
  mut commands: Commands,
  // asset_server: Res<AssetServer>,
  mut meshes: ResMut<Assets<Mesh>>,
  promise_query: Query<(Entity, &PromiseMesh)>,
)
{
  for (entity, PromiseMesh(collider_handler)) in promise_query.iter() {
    if let Some(mesh) = meshes.get(collider_handler) {
      let collider = Collider::trimesh_from_mesh(mesh).unwrap();
      println!("{:#?}", collider);
      commands.entity(entity).insert(RigidBody::Static);
      commands.entity(entity).insert(Transform::from_scale(Vec3::splat(16.88806915283203)));
      commands.entity(entity).insert(collider);
      commands.entity(entity).remove::<PromiseMesh>();
    } else {
      println!("poka sosi");
    }
  }
}
