mod extend_commands;
use bevy::window::*;

use bevy_egui::{egui::{self, Color32}, EguiContexts, EguiPlugin, EguiSettings };

use bevy::{prelude::*, ecs::system::EntityCommands, 
  diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::{math::*, prelude::*};

const PRIMARY_CAMERA_ORDER: isize = 3;
const SECONDARY_CAMERA_ORDER: isize = 2;

fn main() {
    env_logger::init();

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
          primary_window: Window { 
            title: "Game of Life".to_string(),
            // this is need's for stable fps
            present_mode: PresentMode::AutoNoVsync,
            ..default()
          }.into(),
        ..default()
        }),
        EguiPlugin,
        PhysicsPlugins::default(),
        WorldInspectorPlugin::default(),
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin::default(),
    ));

    app.add_systems(Startup, setup_scene);
    app.add_systems(Update, move_players);
    app.add_systems(Update, (ui,  player_respawn));

    app.run();
}

const PLAYER_MOVE_SPEED: f32 = 0.03;
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
) {
    if let Ok(mut lin_vel) = query.get_single_mut() {
      if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
          lin_vel.z -= PLAYER_MOVE_SPEED;
      }
      if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
          lin_vel.z += PLAYER_MOVE_SPEED;
      }
      if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
          lin_vel.x -= PLAYER_MOVE_SPEED;
      }
      if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
          lin_vel.x += PLAYER_MOVE_SPEED;
      }
      
      // Switch the camera order
      if keyboard_input.just_pressed(KeyCode::Space) {
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
            material: materials.add(Color::rgb(0.7, 0.7, 0.8).into()),
            transform: Transform::from_scale(Vec3::new(10.0, 1.0, 10.0)),
            ..default()
        },
        RigidBody::Static,
        Position(Vec3::ZERO),
        Collider::cuboid(10.0, 1.0, 10.0),
    ));

    let camera_entity = commands.spawn_camera(PRIMARY_CAMERA_ORDER).id();
    commands.spawn_player().push_children(&[camera_entity]);

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
    commands.spawn_camera(SECONDARY_CAMERA_ORDER);
}

#[derive(Component)]
pub struct Player;

extend_commands!(
  spawn_camera(order: isize),
  |world: &mut World, entity_id: Entity, order: isize| {
    world
     .entity_mut(entity_id)
     .insert(
    Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
          order: order,
          ..default()
        },
        ..Default::default()
    });
  }
);

extend_commands!(
  spawn_player(),
  |world: &mut World, entity_id: Entity| {
    let pos = Vec3::new(0., 3., 0.);

    let mesh = world.resource_mut::<Assets<Mesh>>().add(Mesh::from(shape::Cube { size: 1. }));
    let material = world.resource_mut::<Assets<StandardMaterial>>().add(Color::rgb(0.8, 0.7, 0.6).into());

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
