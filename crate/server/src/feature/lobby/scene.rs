use std::env;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use shared::feature::lobby::PLANE_SIZE;

pub struct ScenePlugins;

impl Plugin for ScenePlugins {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_scene)
      /*.add_systems(Update, find_and_make_collider)*/;
  }
}

fn setup_scene(mut commands: Commands) {
  // plane
  commands.spawn(
    // server plane got a collider & rigit body
    (
      Friction::new(0.4),
      RigidBody::Static,
      Collider::cuboid(PLANE_SIZE, 0.002, PLANE_SIZE),
    ),
  );
  // commands.spawn(PbrBundle {
  //   mesh: mesh.get_handle("path/to/mesh.gltf#Mesh0"),
  //   material: material.some_material.clone(),
  //   transform: Transform::from_xyz(2.0, 0.0, -5.0),
  //   ..Default::default()
  // });
}

fn spawn_gltf(
  mut commands: Commands,
  ass: Res<AssetServer>,
) {
  // note that we have to include the `Scene0` label
  let my_gltf_mesh = ass.load("terrain.glb#Mesh0");

  commands.spawn((
    PbrBundle {
      mesh: my_gltf_mesh.clone(),
      transform: Transform::from_xyz(2.0, 0.0, -5.0),
      ..Default::default()
    },
    PromisedCollider(my_gltf_mesh)
  ));
}

#[derive(Component)]
struct PromisedCollider(Handle<Mesh>);

fn find_and_make_collider (
  mut commands: Commands,
  mesh: Res<Assets<Mesh>>,
  promise_query: Query<(Entity, &PromisedCollider)>
) {
 /* for (entity, PromisedCollider(collider_handler)) in promise_query.iter() {
    if let Some(mesh) = mesh.get(collider_handler) {
      let collider = Collider::trimesh_from_bevy_mesh(mesh).expect("your code is shit");
      commands.entity(entity).remove::<PromisedCollider>();
      commands.entity(entity).insert((
        Friction::new(0.4),
        RigidBody::Static,
        collider,
      ));
    } else {
      println!("not loaded yet")
    }
  }*/
}

// fn spawn_gltf(
//   mut commands: Commands,
//   ass: Res<AssetServer>,
// ) {
//   // note that we have to include the `Scene0` label
//   let my_gltf = ass.load("house.glb#Scene0");
//
//   // to position our 3d model, simply use the Transform
//   // in the SceneBundle
//   commands.spawn(SceneBundle {
//     scene: my_gltf,
//     transform: Transform::from_xyz(2.0, 0.0, -5.0),
//     ..Default::default()
//   });
// }
