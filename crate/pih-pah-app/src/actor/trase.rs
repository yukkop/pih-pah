use bevy::{ecs::{world::World, entity::Entity}, pbr::{PbrBundle, StandardMaterial}, core::Name, render::{mesh::{Mesh, shape}, color::Color}, asset::Assets, prelude::default};
use bevy::{ecs::system::EntityCommands, prelude::*};
use crate::extend_commands;

extend_commands!(
  spawn_trasepoint(),
  |world: &mut World, entity_id: Entity| {
    let mesh = world
        .resource_mut::<Assets<Mesh>>()
        .add(Mesh::try_from(shape::Icosphere { radius: 0.1, subdivisions: 3 }).unwrap());
    let material = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: Color::RED.into(),
            ..default()
        });

    world
      .entity_mut(entity_id)
      .insert((
        PbrBundle {
          mesh,
          material,
          ..default()
        },
        Name::new("trasepoint"),
      ));
  }
);