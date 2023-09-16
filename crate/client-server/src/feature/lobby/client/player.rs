use crate::feature::lobby::{PLAYER_SIZE, PLAYER_SPAWN_POINT};
use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::extend_commands;

extend_commands!(
  spawn_client_side_player(),
  |world: &mut World, entity_id: Entity| {
    let mesh = world
      .resource_mut::<Assets<Mesh>>()
      .add(Mesh::from(shape::Cube { size: PLAYER_SIZE }));
    let material = world
      .resource_mut::<Assets<StandardMaterial>>()
      .add(Color::rgb(0.8, 0.7, 0.6).into());

    world.entity_mut(entity_id).insert(PbrBundle {
      mesh,
      material,
      transform: Transform::from_translation(PLAYER_SPAWN_POINT),
      ..Default::default()
    });
  }
);
