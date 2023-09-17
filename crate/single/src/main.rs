mod extend_commands;
use bevy::window::*;
use bevy::app::AppExit;

use bevy_egui::{egui::{self, Color32}, EguiContexts, EguiPlugin};

use bevy::{prelude::*, ecs::system::EntityCommands, 
  diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::{math::*, prelude::*};

const PRIMARY_CAMERA_ORDER: isize = 3;
const SECONDARY_CAMERA_ORDER: isize = 2;

#[derive(Resource)]
pub struct CameraAngle(Quat);

// parent point where camera look at
#[derive(Component)]
struct PlayerCamera;

fn main() {
    env_logger::init();

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
          primary_window: Window { 
            title: "Game of Life".to_string(),
            // this is need's for stable fps
            // present_mode: PresentMode::AutoNoVsync,
            ..default()
          }.into(),
        ..default()
        }),
        EguiPlugin,
        PhysicsPlugins::default(),
        WorldInspectorPlugin::default(),
        // LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin::default(),
    ));

    app.add_systems(Startup, setup_scene);

    // TODO need more test
    // This system is need be not depend on delta_seconds
    app.add_systems(FixedUpdate, move_players);
    // but this system is need be more smooth, 
    // else camera will be not stable when player move
    // but some times camera will be not stable anyway... need more test
    app.add_systems(Update, stabilize_camera);
    // it can be fixed on client server architecture
    // becosue client will be not have physics
    // so render camera pivot and player will be same

    app.add_systems(FixedUpdate, player_respawn);
    app.add_systems(Update, ui);


    app.run();
}

const PLAYER_MOVE_SPEED: f32 = 0.15;
const PLAYER_CAMERA_ROTATION_SPEED: f32 = 0.01;
const PLAYER_SPAWN_POINT: Vec3 = Vec3::new(0., 10., 0.);

fn player_respawn(
  _commands: Commands,
  mut query: Query<(&mut Position, &mut LinearVelocity, &Player)>,
) {
  for (mut position, mut linear_velocity, _player) in query.iter_mut() {
    if position.y < -5. {
      position.x = PLAYER_SPAWN_POINT.x;
      position.y = PLAYER_SPAWN_POINT.y;
      position.z = PLAYER_SPAWN_POINT.z;

      linear_velocity.z = 0.;
      linear_velocity.y = 0.;
      linear_velocity.x = 0.;
    }
  }
}

fn move_players(
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<&mut LinearVelocity, With<Player>>,
  mut camera_query: Query<&mut Camera>,
  mut camera_pivot_query: Query<&mut Transform, With<PlayerCamera>>,
  playe_global_transform_query: Query<&GlobalTransform, With<PlayerCamera>>,
  mut exit: EventWriter<AppExit>,
) {
    if let Ok(mut lin_vel) = query.get_single_mut() {

      let global_transform = playe_global_transform_query.single();
      if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        let global_forward_4d = global_transform.compute_matrix() * Vec3::NEG_Z.extend(0.0);
        lin_vel.x += global_forward_4d.x * PLAYER_MOVE_SPEED;
        lin_vel.z += global_forward_4d.z * PLAYER_MOVE_SPEED;
      }
      if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        let global_forward_4d = global_transform.compute_matrix() * Vec3::Z.extend(0.0);
        lin_vel.x += global_forward_4d.x * PLAYER_MOVE_SPEED;
        lin_vel.z += global_forward_4d.z * PLAYER_MOVE_SPEED;
      }
      if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        let global_forward_4d = global_transform.compute_matrix() * Vec3::NEG_X.extend(0.0);
        lin_vel.x += global_forward_4d.x * PLAYER_MOVE_SPEED;
        lin_vel.z += global_forward_4d.z * PLAYER_MOVE_SPEED;
      }
      if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        let global_forward_4d = global_transform.compute_matrix() * Vec3::X.extend(0.0);
        lin_vel.x += global_forward_4d.x * PLAYER_MOVE_SPEED;
        lin_vel.z += global_forward_4d.z * PLAYER_MOVE_SPEED;
      }

      // camera rotation
      if keyboard_input.pressed(KeyCode::E) {
        if let Ok(mut transform) = camera_pivot_query.get_single_mut() {
          let rotation = Quat::from_rotation_y(PI * PLAYER_CAMERA_ROTATION_SPEED /* * delta_seconds */);
          transform.rotation *= rotation;
        }
      }
      if keyboard_input.pressed(KeyCode::Q) {
        if let Ok(mut transform) = camera_pivot_query.get_single_mut() {
          let rotation = Quat::from_rotation_y(PI * PLAYER_CAMERA_ROTATION_SPEED * -1. /* * delta_seconds */);
          transform.rotation *= rotation;
        }
      }

      if keyboard_input.just_pressed(KeyCode::Escape)
        || keyboard_input.pressed(KeyCode::ControlLeft)
        && keyboard_input.just_pressed(KeyCode::C)  {
        exit.send(AppExit);
      }
      
      // Switch the camera order
      if keyboard_input.just_pressed(KeyCode::C) {
        for mut camera in camera_query.iter_mut() {
          if camera.order == SECONDARY_CAMERA_ORDER {
            camera.order = PRIMARY_CAMERA_ORDER;
          } else if camera.order == PRIMARY_CAMERA_ORDER {
            camera.order = SECONDARY_CAMERA_ORDER;
          }
        }
      }
    }
}

fn ui(mut contexts: EguiContexts,
  diagnostics: Res<DiagnosticsStore>,
      ) {
    let (mut raw, mut sma, mut ema): (String, String, String) = ("raw: ".into(), "sma: ".into(), "ema:".into());
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(raw_value) = fps.value() {
            raw = format!("raw: {raw_value:.2}");
        }
        if let Some(sma_value) = fps.average() {
            sma = format!("sma: {sma_value:.2}");
        }
        if let Some(ema_value) = fps.smoothed() {
            ema = format!("ema: {ema_value:.2}");
        }
    };

    let ctx = contexts.ctx_mut();

    let my_frame = egui::containers::Frame {
              fill: Color32::TRANSPARENT,
              ..default()
          };

    egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {
        ui.label(raw);
        ui.label(sma);
        ui.label(ema);
    });
}

// system for camera follow player
fn stabilize_camera(
mut camera_pivot_query: Query<&mut Transform, With<PlayerCamera>>,
palyer_query: Query<&Position, With<Player>>,
) {
  if let Ok(mut camera_transform) = camera_pivot_query.get_single_mut() {
    if let Ok(player_position) = palyer_query.get_single() {
      println!("stabilize_camera: {:?}", player_position.0);
      camera_transform.translation = player_position.0;
    }
    else {
      println!("stabilize_camera: player not found");
    }
  } else {
    println!("stabilize_camera: camera not found");
  }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
  ) {
    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // plane
    commands.spawn((
         PbrBundle {
            mesh: cube_mesh.clone(),
            material: materials.add(Color::rgba(0.7, 0.7, 0.8, 1.).into()),
            transform: Transform::from_scale(Vec3::new(10.0, 1.0, 10.0)),
            ..default()
        },
        RigidBody::Static,
        Position(Vec3::ZERO),
        Collider::cuboid(10.0, 1.0, 10.0),
    ));

    let camera_entity = commands.spawn((
      Camera3dBundle {
        transform: Transform::from_xyz(0., 10., 15.).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
          order: PRIMARY_CAMERA_ORDER,
          ..default()
        },
        ..Default::default()
      },
    )).id();

    commands.spawn((
      PlayerCamera,
      // it is need for camera render correct, do not understand why
      PbrBundle {
        mesh: cube_mesh,
        material: materials.add(Color::rgba(0.7, 0.1, 0.2, 0.).into()),
        transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.3)),
        ..Default::default()
      },
    )).push_children(&[camera_entity]);

    commands.spawn_player();

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands.spawn(
      Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
          order: SECONDARY_CAMERA_ORDER,
          ..default()
        },
        ..Default::default()
    });
}

#[derive(Component)]
pub struct Player;

extend_commands!(
  spawn_player(),
  |world: &mut World, entity_id: Entity| {
    let pos = Vec3::new(0., 3., 0.);

    let mesh = world.resource_mut::<Assets<Mesh>>().add(Mesh::from(shape::Cube { size: 1. }));
    let material = world.resource_mut::<Assets<StandardMaterial>>().add(Color::rgba(0.8, 0.7, 0.6, 1.).into());

    world
      .entity_mut(entity_id)
     .insert((
       PbrBundle {
         mesh: mesh,
         material: material,
         // transform: Transform::from_translation(PLAYER_SPAWN_POINT),
         ..Default::default()
       },
       RigidBody::Dynamic,
       Position(pos),
       Collider::cuboid(1., 1., 1.),
       Player,
     ));
  }
);
