use bevy::{prelude::*, ecs::system::EntityCommands};
use crate::lib::{PLAYER_MOVE_SPEED, PlayerInput};
use crate::extend_commands;
use renet::ClientId;
use bevy_xpbd_3d::prelude::*;

pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
    fn build(&self, app: &mut App) {
        app
          .add_systems(FixedUpdate, move_players_system);
    }
}

fn move_players_system(mut query: Query<(&mut LinearVelocity, &PlayerInput)>, time: Res<Time>) {
  for (mut linear_velocity, input) in query.iter_mut() {
      let x = (input.right as i8 - input.left as i8) as f32;
      let y = (input.down as i8 - input.up as i8) as f32;
      // transform.translation.x += x * PLAYER_MOVE_SPEED * time.delta().as_secs_f32();
      // transform.translation.z += y * PLAYER_MOVE_SPEED * time.delta().as_secs_f32();
      linear_velocity.0.x += x * PLAYER_MOVE_SPEED * time.delta().as_secs_f32();
      linear_velocity.0.z += y * PLAYER_MOVE_SPEED * time.delta().as_secs_f32();
  }
}

extend_commands!(
  spawn_player(client_id: ClientId),
  |world: &mut World, entity_id: Entity, client_id: ClientId| {
    use crate::lib::Player;
    use crate::lib::{PLAYER_SIZE, PLAYER_SPAWN_POINT};

    world
     .entity_mut(entity_id)
     .insert((
       RigidBody::Dynamic,
       Position(PLAYER_SPAWN_POINT),
       Collider::cuboid(PLAYER_SIZE, PLAYER_SIZE, PLAYER_SIZE),
     ))
     .insert(PlayerInput::default())
     .insert(Player { id: client_id });
  }
);
