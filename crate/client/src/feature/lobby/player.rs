use bevy::{ecs::system::EntityCommands, prelude::*};
use shared::feature::lobby::{PLAYER_SIZE, PLAYER_SPAWN_POINT};

use shared::extend_commands;

// rgb(0.8, 0.7, 0.6)

extend_commands!(
  spawn_client_side_player(color: Color),
  |world: &mut World, entity_id: Entity, color: Color| {
    let mesh = world
      .resource_mut::<Assets<Mesh>>()
      // TODO: Have a resource with shared mesh list instead of adding meshes each time
      .add(Mesh::from(shape::Cube { size: PLAYER_SIZE }));
    let material = world
      .resource_mut::<Assets<StandardMaterial>>()
      .add(color.into());

    world.entity_mut(entity_id).insert(PbrBundle {
      mesh,
      material,
      transform: Transform::from_translation(PLAYER_SPAWN_POINT),
      ..Default::default()
    });
  }
);
