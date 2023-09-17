use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::extend_commands;

const PRIMARY_CAMERA_ORDER: isize = 3;
const SECONDARY_CAMERA_ORDER: isize = 2;

pub fn camera_switch(
  keyboard_input: Res<Input<KeyCode>>,
  mut camera_query: Query<&mut Camera>, /* , time: Res<Time> */
) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    for mut camera in camera_query.iter_mut() {
      // Switch the camera order
      if camera.order == 3 {
        camera.order = 2;
      } else if camera.order == 2 {
        camera.order = 3;
      }
    }
  }
}

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
