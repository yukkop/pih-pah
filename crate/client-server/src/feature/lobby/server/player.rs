use crate::extend_commands;
use crate::feature::multiplayer::{Player, PlayerInput, PlayerViewDirrection};
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_xpbd_3d::prelude::*;
use renet::ClientId;

use crate::feature::lobby::{
  PLAYER_CAMERA_ROTATION_SPEED, PLAYER_MOVE_SPEED, PLAYER_SIZE, PLAYER_SPAWN_POINT,
};

pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
  fn build(&self, app: &mut App) {
    app.add_systems(FixedUpdate, (move_players_system, player_respawn));
  }
}

fn move_players_system(
  mut query: Query<(&mut LinearVelocity, &mut PlayerViewDirrection, &PlayerInput)>, /* , time: Res<Time> */
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

extend_commands!(
  spawn_server_side_player(client_id: ClientId),
  |world: &mut World, entity_id: Entity, client_id: ClientId| {
    world
     .entity_mut(entity_id)
     .insert((
       Friction::new(0.4),
       RigidBody::Dynamic,
       Position(PLAYER_SPAWN_POINT),
       Collider::cuboid(PLAYER_SIZE, PLAYER_SIZE, PLAYER_SIZE),
     ))
     .insert(PlayerInput::default())
     .insert(Player { id: client_id })
     .insert(PlayerViewDirrection(Quat::default()));
  }
);
