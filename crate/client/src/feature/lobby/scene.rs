use bevy::gltf::Gltf;
use bevy::prelude::{shape::Plane, *};
use bevy_egui::EguiSet::ProcessInput;
use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};
use shared::feature::lobby::PLANE_SIZE;

pub struct ScenePlugins;

impl Plugin for ScenePlugins {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, (setup_scene, spawn_gltf_mesh));
    // app.register_type::<PromiseMesh>();
    // app.register_type::<PromiseScene>();
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
    // material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
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

fn spawn_gltf_mesh(
  mut commands: Commands,
  ass: Res<AssetServer>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // note that we have to include the `Scene0` label
  // let my_gltf_mesh = ass.load("terrain.glb#MeshPlane.046");
  // let mesh_handle = ass.get_handle("terrain.gltf#Mesh0/Primitive0");
  let mesh_handle = ass.load("cube2m.glb#Mesh0/Primitive0");

  commands.spawn((
    PbrBundle {
      mesh: mesh_handle.clone(),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      transform: Transform::from_xyz(0.0, 0.0, 5.0),
      ..Default::default()
    },
    Name::new("GltfMesh"),
    // PromiseMesh(mesh_handle)
  ));

  let mesh_handle = ass.load("terrain-2.glb#Mesh0/Primitive0");

  commands.spawn((
    PbrBundle {
      mesh: mesh_handle.clone(),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      transform: Transform::from_scale(Vec3::splat(16.88806915283203)),
      ..Default::default()
    },
    Name::new("GltfMesh"),
    // PromiseMesh(mesh_handle)
  ));

  // let scene_handle = ass.load("terrain-2.glb#Scene0");
  //
  // commands.spawn((
  //   SceneBundle {
  //     scene: scene_handle.clone(),
  //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
  //     ..Default::default()
  //   },
  //   Name::new("GltfMesh"),
  //   // PromiseMesh(mesh_handle)
  // ));
}

// fn test(
//   asset_server: Res<AssetServer>,
//   mut meshes: ResMut<Assets<Mesh>>,
//   promise_query: Query<(Entity, &PromiseMesh)>,
// )
// {
//   for (entity, PromiseMesh(collider_handler)) in promise_query.iter() {
//     if let Some(mesh) = meshes.get(&collider_handler) {
//       println!("loaded huli");
//       // Do something with the mesh
//     }
//     else {
//       println!("poka sosi");
//     }
//   }
// }
//
// fn spawn_gltf_scene(
//   mut commands: Commands,
//   ass: Res<AssetServer>,
//   scene: Res<Assets<Scene>>
// ) {
//   // note that we have to include the `Scene0` label
//   let my_gltf = ass.load("terrain.glb#Scene0");
//   if scene.get(&my_gltf).is_some() {
//     println!("afhdglhl;dshgu;heskjghilhroi;gh");
//   }
//
//   // to position our 3d model, simply use the Transform
//   // in the SceneBundle
//   commands.spawn(
//     (
//       SceneBundle {
//         scene: my_gltf.clone(),
//         transform: Transform::from_xyz(0., -5., 0.),
//         ..Default::default()
//       },
//       PromiseScene(my_gltf)
//     )
//   );
// }
//
// #[derive(Component, Debug, Clone, InspectorOptions, Reflect/* , Serialize, Deserialize */)]
// struct PromiseScene(Handle<Scene>);
// #[derive(Component, Debug, Clone, InspectorOptions, Reflect/*, Serialize, Deserialize */)]
// struct PromiseMesh(Handle<Mesh>);
//
//
// fn find_and_make_collider (
//   mut commands: Commands,
//   scene: Res<Assets<Scene>>,
//   mesh: Res<Assets<Mesh>>,
//   promise_query: Query<(Entity, &PromiseScene)>
// ) {
//   for (entity, PromiseScene(collider_handler)) in promise_query.iter() {
//     if let Some(scene_) = scene.get(collider_handler) {
//       println!("loaded!!");
//       // scene_.
//       // mesh.add(Mesh::from(scene_));
//       commands.entity(entity).remove::<PromiseScene>();
//     } else {
//       println!("not loaded yet");
//     }
//   }
// }

// fn get_first_mesh(
//   world: &World, gltf_handle: Handle<SceneFormat>
// ) -> Option<Handle<Mesh>> {
//   let mesh_storage = world.read_resource::<AssetStorage<Mesh>>();
//   let scene_storage = world.read_resource::<AssetStorage<SceneFormat>>();
//
//   if let Some(scene_asset) = scene_storage.get(&gltf_handle) {
//     // Assume the first entity is a mesh
//     if let Some(first_mesh) = scene_asset.meshes.first() {
//       if mesh_storage.get(first_mesh).is_some() {
//         return Some(first_mesh.clone());
//       }
//     }
//   }
//
//   None
// }