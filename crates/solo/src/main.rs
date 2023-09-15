mod extend_commands;
use bevy::render::mesh::shape::Plane;
use bevy::window::*;

use bevy_egui::{egui::{self, Color32}, EguiContexts, EguiPlugin, EguiSettings };

use bevy::{prelude::*, ecs::system::EntityCommands, 
  diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::{math::*, prelude::*};

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
    app.add_systems(Update, (move_players, ui));

    app.run();
}

fn move_players(
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<&mut LinearVelocity, With<Player>>,
  ) {
    for mut lin_vel in &mut query {
        if keyboard_input.pressed(KeyCode::Up) {
            lin_vel.z -= 0.15;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            lin_vel.z += 0.15;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            lin_vel.x -= 0.15;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            lin_vel.x += 0.15;
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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
