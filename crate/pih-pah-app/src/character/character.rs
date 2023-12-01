use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_xpbd_3d::prelude::*;
use renet::ClientId;
use serde::{Serialize, Deserialize};
use crate::extend_commands;
use crate::lobby::{LobbyState, PlayerInput, PlayerViewDirection};
use crate::lobby::Character;
use crate::world::Me;

pub const PLAYER_MOVE_SPEED: f32 = 0.07;
pub const PLAYER_CAMERA_ROTATION_SPEED: f32 = 0.015;
pub const PLAYER_SIZE: f32 = 1.0;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct TiedCamera(Entity);

pub struct CharacterPlugins;

impl Plugin for CharacterPlugins {
    fn build(&self, app: &mut App) {
        app
          .add_systems(
              FixedUpdate,
              move_characters.run_if(not(in_state(LobbyState::None))))
          .add_systems(
              PostUpdate,
              tied_camera_follow.run_if(not(in_state(LobbyState::None))));
    }
}

fn tied_camera_follow(
  mut tied_camera_query: Query<(&TiedCamera, &mut Transform)>,
  view_direction_query: Query<&PlayerViewDirection, With<Me>>,
  transform_query: Query<&Transform, Without<TiedCamera>>,
) {
  for (TiedCamera(target) , mut transform) in tied_camera_query.iter_mut() {
      if let Ok(target_transform) = transform_query.get(*target) {
          transform.translation = target_transform.translation;
          transform.rotation = view_direction_query.single().0;
      }
      else {
          warn!("Tied camera cannot follow object ({:?}) without transform", target)
      }
  }
}

fn move_characters(
    mut query: Query<(&mut LinearVelocity, &mut PlayerViewDirection, &PlayerInput)>, /* , time: Res<Time> */
) {
    for (mut linear_velocity, mut view_direction, input) in query.iter_mut() {
        let dx = (input.right as i8 - input.left as i8) as f32;
        let dy = (input.down as i8 - input.up as i8) as f32;

        let jumped = input.jump;

        // convert axises to global
        let global_x = view_direction.0.mul_vec3(Vec3::X);
        let global_y = view_direction.0.mul_vec3(Vec3::Z);

        // never use delta time in fixed update !!!

        // move by x axis
        linear_velocity.x +=
            dx * PLAYER_MOVE_SPEED * global_x.x * 1.5_f32.powf(input.sprint as i32 as f32); // * time.delta().as_secs_f32();
        linear_velocity.z +=
            dx * PLAYER_MOVE_SPEED * global_x.z * 1.5_f32.powf(input.sprint as i32 as f32); // * time.delta().as_secs_f32();

        // move by y axis
        linear_velocity.x +=
            dy * PLAYER_MOVE_SPEED * global_y.x * 1.5_f32.powf(input.sprint as i32 as f32); // * time.delta().as_secs_f32();
        linear_velocity.z +=
            dy * PLAYER_MOVE_SPEED * global_y.z * 1.5_f32.powf(input.sprint as i32 as f32); // * time.delta().as_secs_f32();

        if jumped && linear_velocity.y.abs() <= 0.03 {
            linear_velocity.y = 5.0; //velocity = sqrt(2 * 9.8(g) * height), но лучше ставить немного больше
        }

        // camera turn
        let turn = (input.turn_right as i8 - input.turn_left as i8) as f32;

        let rotation = Quat::from_rotation_y(
            PLAYER_CAMERA_ROTATION_SPEED * turn, /* * delta_seconds */
        );
        view_direction.0 *= rotation;

    }

}

extend_commands!(
  spawn_character(client_id: ClientId, color: Color, spawn_point: Vec3),
  |world: &mut World, entity_id: Entity, client_id: ClientId, color: Color, spawn_point: Vec3| {

    let mesh = world
      .resource_mut::<Assets<Mesh>>()
      // TODO: Have a resource with shared mesh list instead of adding meshes each time
      .add(Mesh::from(shape::Cube { size: PLAYER_SIZE }));
    let material = world
      .resource_mut::<Assets<StandardMaterial>>()
      .add(color.into());

    world
     .entity_mut(entity_id)
     .insert((
       PbrBundle {
          mesh,
          material,
          ..Default::default()
       },
       Friction::new(0.4),
       RigidBody::Dynamic,
       Position::from_xyz(spawn_point.x, spawn_point.y, spawn_point.z),
       Collider::cuboid(PLAYER_SIZE, PLAYER_SIZE, PLAYER_SIZE),
     ))
     .insert(PlayerInput::default())
     .insert(Character { id: client_id })
     .insert(PlayerViewDirection(Quat::default()));
  }
);

extend_commands!(
  spawn_tied_camera(target: Entity),
  |world: &mut World, entity_id: Entity, target: Entity| {
    world
      .entity_mut(entity_id)
      .insert((
        // TODO find light prd without mesh
        PbrBundle::default(),
        TiedCamera(target),
        Name::new("TiedCamera"),
      ))
      .with_children(|parent| {
        // spawn tied camera
        parent.spawn(Camera3dBundle {
          transform: Transform::from_xyz(0., 10., 15.).looking_at(Vec3::ZERO, Vec3::Y),
          ..Default::default()
        });
      });

  }
);