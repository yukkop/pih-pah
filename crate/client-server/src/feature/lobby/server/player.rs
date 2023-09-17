use crate::extend_commands;
use crate::feature::multiplayer::{Player, PlayerInput, PlayerViewDirrection};
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_xpbd_3d::prelude::*;
use renet::ClientId;

use crate::feature::lobby::{PLAYER_MOVE_SPEED, PLAYER_SIZE, PLAYER_SPAWN_POINT, PLAYER_CAMERA_ROTATION_SPEED};

pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
  fn build(&self, app: &mut App) {
    app.add_systems(FixedUpdate, (move_players_system, player_respawn));
  }
}

fn move_players_system(
  mut query: Query<(&mut LinearVelocity, &mut PlayerViewDirrection, &PlayerInput)>, /* , time: Res<Time> */
) {
  for (mut linear_velocity, mut view_dirrection, input) in query.iter_mut() {
    let x = (input.right as i8 - input.left as i8) as f32;
    let y = (input.down as i8 - input.up as i8) as f32;

    // convert axises to global
    let global_x = view_dirrection.0.mul_vec3(Vec3::X);
    let global_y = view_dirrection.0.mul_vec3(Vec3::Z);

    // never use delta time in fixed update !!!

    // move by x axis
    linear_velocity.x += x * PLAYER_MOVE_SPEED * global_x.x; // * time.delta().as_secs_f32();
    linear_velocity.z += x * PLAYER_MOVE_SPEED * global_x.z; // * time.delta().as_secs_f32();

    // move by y axis
    linear_velocity.x += y * PLAYER_MOVE_SPEED * global_y.x; // * time.delta().as_secs_f32();
    linear_velocity.z += y * PLAYER_MOVE_SPEED * global_y.z; // * time.delta().as_secs_f32();

    // camera turn
    let turn = (input.turn_right as i8 - input.turn_left as i8) as f32;

    let rotation = Quat::from_rotation_y(PLAYER_CAMERA_ROTATION_SPEED * turn /* * delta_seconds */);
    view_dirrection.0 *= rotation;
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
       RigidBody::Dynamic,
       Position(PLAYER_SPAWN_POINT),
       Collider::cuboid(PLAYER_SIZE, PLAYER_SIZE, PLAYER_SIZE),
     ))
     .insert(PlayerInput::default())
     .insert(Player { id: client_id })
     .insert(PlayerViewDirrection(Quat::default()));
  }
);
