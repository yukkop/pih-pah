use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::extend_commands;

extend_commands!(spawn_camera(), |world: &mut World, entity_id: Entity| {
  world.entity_mut(entity_id).insert(Camera3dBundle {
    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    camera: Camera {
      order: 3,
      ..default()
    },
    ..default()
  });
});
