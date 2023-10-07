#![allow(unused_doc_comments)]
use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::extend_commands;

const PRIMARY_CAMERA_ORDER: isize = 3;
const SECONDARY_CAMERA_ORDER: isize = 2;

// it is not dependet to server
pub fn camera_switch(keyboard_input: Res<Input<KeyCode>>, mut camera_query: Query<&mut Camera>) {
  if keyboard_input.just_pressed(KeyCode::C) {
    for mut camera in camera_query.iter_mut() {
      // Switch the camera order
      if camera.order == PRIMARY_CAMERA_ORDER {
        camera.order = SECONDARY_CAMERA_ORDER;
      } else if camera.order == SECONDARY_CAMERA_ORDER {
        camera.order = PRIMARY_CAMERA_ORDER;
      }
    }
  }
}

extend_commands!(
  spawn_spectator_camera(),
  |world: &mut World, entity_id: Entity| {
    world.entity_mut(entity_id).insert(Camera3dBundle {
      transform: Transform::from_xyz(-5.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      camera: Camera {
        order: SECONDARY_CAMERA_ORDER,
        ..default()
      },
      ..default()
    });
  }
);

#[derive(Component)]
pub struct TiedCamera;

/// The camera is anchored to the object but
/// features stabilization to maintain a consistent viewing angle,
/// irrespective of the object's position.
extend_commands!(
  spawn_tied_camera(),
  |world: &mut World, entity_id: Entity| {
    // let camera_entity = commands.spawn((
    //   Camera3dBundle {
    //     transform: Transform::from_xyz(0., 10., 15.).looking_at(Vec3::ZERO, Vec3::Y),
    //     camera: Camera {
    //       order: PRIMARY_CAMERA_ORDER,
    //       ..default()
    //     },
    //     ..Default::default()
    //   },
    // )).id();
    //
    // commands.spawn((
    //   PlayerCamera,
    //   // it is need for camera render correct, do not understand why
    //   PbrBundle {
    //     mesh: cube_mesh,
    //     material: materials.add(Color::rgba(0.7, 0.1, 0.2, 0.).into()),
    //     transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.3)),
    //     ..Default::default()
    //   },
    // )).push_children(&[camera_entity]);

    // spawn pivot
    world
      .entity_mut(entity_id)
      .insert((
        // Without the PRD, child elements are not rendered.
        // TODO find light prd without mesh
        PbrBundle::default(),
        TiedCamera,
      ))
      .with_children(|parent| {
        // spawn tied camera
        parent.spawn(Camera3dBundle {
          transform: Transform::from_xyz(0., 10., 15.).looking_at(Vec3::ZERO, Vec3::Y),
          camera: Camera {
            order: PRIMARY_CAMERA_ORDER,
            ..default()
          },
          ..Default::default()
        });
      });

    // world.entity_mut(entity_id).insert(Camera3dBundle {
    //   transform: Transform::from_xyz(0., 10., 15.).looking_at(Vec3::ZERO, Vec3::Y),
    //   camera: Camera {
    //     order: PRIMARY_CAMERA_ORDER,
    //     ..default()
    //   },
    //   ..default()
    // });
  }
);
