use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::gltf::GltfPlugin;
use bevy::prelude::*;
use bevy::render::mesh::MeshPlugin;
use bevy::scene::ScenePlugin;
use bevy::window::{PresentMode, Window, WindowPlugin};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::InspectorOptions;
use bevy_xpbd_3d::prelude::*;

use server::feature::lobby::LobbyPlugins;
use server::feature::multiplayer::MultiplayerPlugins;
// use server::feature::UiDebugPlugins;

use server::feature::heartbeat::HeartbeatPlugins;
use shared::feature::multiplayer::panic_on_error_system;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shared::lib::netutils::{is_http_address, is_ip_with_port};

fn main() {
  std::env::set_var(
    "RUST_LOG",
    std::env::var("RUST_LOG").unwrap_or(String::from("info")),
  );
  env_logger::init();

  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 || &args[1] == "-h" || &args[1] == "--help" {
    println!("Usage: ");
    println!("  server '<server address>' '<load-balancer address>'");
    panic!("Not enough arguments.");
  }

  // to listen clients
  let listen_addr = match &args[1] {
    addr if is_http_address(addr) => addr,
    addr if is_ip_with_port(addr) => addr,
    _ => panic!("Invalid argument, must be an HTTP address or an IP with port.")
  };

  let is_debug = std::env::var("DEBUG").is_ok();

  let mut app = App::new();

  let window_plugin_override = WindowPlugin {
    primary_window: Some(Window {
      title: "pih-pah".to_string(),
      // this is needed for stable fps
      present_mode: PresentMode::AutoNoVsync,
      ..default()
    }),
    ..default()
  };

  if !is_debug {
    // Normal plugins
    app
        .add_plugins(MinimalPlugins)
        .add_plugins(AssetPlugin::default())
        .add_plugins(MeshPlugin)
        .add_plugins(ScenePlugin)
        .add_plugins(GltfPlugin::default());

  } else {
    // Debug plugins
    app.add_plugins(DefaultPlugins.set(window_plugin_override))
      .add_plugins(EguiPlugin)
      //.add_plugins(UiDebugPlugins);
      .add_plugins(LogDiagnosticsPlugin::default())
      //.add_plugins(FrameTimeDiagnosticsPlugin);
      .add_plugins(WorldInspectorPlugin::default());
  }

  if args.len() >= 3 {
    let addr = match &args[2] {
      addr if is_http_address(addr) => addr,
      addr if is_ip_with_port(addr) => addr,
      _ => panic!("Invalid argument, must be an HTTP address or an IP with port."),
    };

    // to send online reports to main server
    app.add_plugins(HeartbeatPlugins::by_string(
      addr.clone().to_string(),
      listen_addr.to_string(),
    ));
  }

  // Plugins that's always there
  app.add_plugins(LobbyPlugins)
    .add_plugins(PhysicsPlugins::default())
    .add_plugins(MultiplayerPlugins::by_string(listen_addr.to_string()))
    .add_systems(Update, panic_on_error_system)
    .add_systems(Startup, spawn_gltf_mesh)
    .add_systems(Update, test)
    .run();

}

fn spawn_gltf_mesh(
  mut commands: Commands,
  ass: Res<AssetServer>,
) {
  let mesh_handle = ass.load("terrain-2.glb#Mesh0/Primitive0");

  commands.spawn((
    Name::new("GltfMesh"),
    PromiseMesh(mesh_handle)
  ));
}

fn test(
  asset_server: Res<AssetServer>,
  mut meshes: ResMut<Assets<Mesh>>,
  promise_query: Query<(Entity, &PromiseMesh)>,
)
{
  for (entity, PromiseMesh(collider_handler)) in promise_query.iter() {
    if let Some(mesh) = meshes.get(&collider_handler) {
      println!("loaded huli");
      // Do something with the mesh
    }
    else {
      println!("poka sosi");
    }
  }
}
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
#[derive(Component, Debug, Clone, InspectorOptions, Reflect/* , Serialize, Deserialize */)]
struct PromiseScene(Handle<Scene>);
#[derive(Component, Debug, Clone, InspectorOptions, Reflect/*, Serialize, Deserialize */)]
struct PromiseMesh(Handle<Mesh>);
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
