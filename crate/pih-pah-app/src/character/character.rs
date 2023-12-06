use crate::component::{AxisName, DespawnReason, Respawn, UntouchedTimerValue};
use crate::extend_commands;
use crate::lobby::Character;
use crate::lobby::{LobbyState, PlayerId, PlayerInput, PlayerViewDirection};
use crate::world::{Me, MyLayers};
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_xpbd_3d::prelude::*;
use serde::{Deserialize, Serialize};

pub const PLAYER_MOVE_SPEED: f32 = 0.07;
pub const PLAYER_CAMERA_ROTATION_SPEED: f32 = 0.015;
pub const PLAYER_SIZE: f32 = 2.0;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct TiedCamera(Entity);

#[derive(Component, Debug)]
struct JumpHelper {
    last_viable_normal: Vec3,
}

pub struct CharacterPlugins;

impl Plugin for CharacterPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            move_characters.run_if(
                not(in_state(LobbyState::None)).and_then(not(in_state(LobbyState::Client))),
            ),
        )
        .add_systems(
            FixedUpdate,
            jump.run_if(
                not(in_state(LobbyState::None)).and_then(not(in_state(LobbyState::Client))),
            ),
        )
        .add_systems(
            PostUpdate,
            tied_camera_follow.run_if(not(in_state(LobbyState::None))),
        )
        .add_systems(
            FixedUpdate,
            update_jump_normals.run_if(
                not(in_state(LobbyState::None)).and_then(not(in_state(LobbyState::Client))),
            ),
        );
    }
}

fn tied_camera_follow(
    mut tied_camera_query: Query<(&TiedCamera, &mut Transform)>,
    view_direction_query: Query<&PlayerViewDirection, With<Me>>,
    transform_query: Query<&Transform, Without<TiedCamera>>,
) {
    for (TiedCamera(target), mut transform) in tied_camera_query.iter_mut() {
        if let Ok(target_transform) = transform_query.get(*target) {
            transform.translation = target_transform.translation;
            if let Ok(view_dirrection) = view_direction_query.get_single() {
                transform.rotation = view_dirrection.0;
            }
        } else {
            warn!(
                "Tied camera cannot follow object ({:?}) without transform",
                target
            )
        }
    }
}

fn update_jump_normals(
    mut query: Query<(&mut JumpHelper, Entity, &GlobalTransform)>,
    collisions: Res<Collisions>,
) {
    let mut direction_vector = Vec3::ZERO;
    for (mut jump_direction, player_entity, transform) in query.iter_mut() {
        for collision in collisions.collisions_with_entity(player_entity) {
            for manifold in collision.manifolds.iter() {
                for contact in manifold.contacts.iter() {
                    direction_vector += transform.translation() - contact.point1;
                }
            }
        }
        jump_direction.last_viable_normal = direction_vector.normalize_or_zero();
    }
}

fn jump(
    mut query: Query<(&mut LinearVelocity, &PlayerInput, Entity, &JumpHelper)>, /* , time: Res<Time> */
    gravity: Res<Gravity>,
    collisions: Res<Collisions>,
) {
    for (mut linear_velocity, input, player_entity, jump_direction) in query.iter_mut() {
        let jumped = input.jump;
        if jumped
            && collisions
                .collisions_with_entity(player_entity)
                .next()
                .is_some()
        {
            **linear_velocity +=
                jump_direction.last_viable_normal * (-gravity.0.y * 2.0 * PLAYER_SIZE).sqrt() * 1.1; // sqrt(2gh)
            log::info!("{:?}", jump_direction.last_viable_normal);
        }
    }
}

fn move_characters(
    mut query: Query<(&mut LinearVelocity, &mut PlayerViewDirection, &PlayerInput)>, /* , time: Res<Time> */
) {
    for (mut linear_velocity, mut view_direction, input) in query.iter_mut() {
        let dx = (input.right as i8 - input.left as i8) as f32;
        let dy = (input.down as i8 - input.up as i8) as f32;

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

        // camera turn
        let turn = (input.turn_right as i8 - input.turn_left as i8) as f32;

        let rotation = Quat::from_rotation_y(
            PLAYER_CAMERA_ROTATION_SPEED * turn, /* * delta_seconds */
        );
        view_direction.0 *= rotation;
    }
}

extend_commands!(
  spawn_character(player_id: PlayerId, color: Color, spawn_point: Vec3),
  |world: &mut World, entity_id: Entity, player_id: PlayerId, color: Color, spawn_point: Vec3| {

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
       JumpHelper{last_viable_normal: Vec3::Y},
       CollisionLayers::new([MyLayers::Default], [MyLayers::Default, MyLayers::ActorNoclip]),
     ))
     .insert(Respawn::new(DespawnReason::Less(-10., AxisName::Y), spawn_point, UntouchedTimerValue::Timer(10.)))
     .insert(PlayerInput::default())
     .insert(Character { id: player_id })
     .insert(PlayerViewDirection(Quat::default()));
  }
);

extend_commands!(
  spawn_character_shell(color: Color, spawn_point: Vec3),
  |world: &mut World, entity_id: Entity, color: Color, spawn_point: Vec3| {

    let mesh = world
      .resource_mut::<Assets<Mesh>>()
      // TODO: Have a resource with shared mesh list instead of adding meshes each time
      .add(Mesh::from(shape::Cube { size: PLAYER_SIZE }));
    let material = StandardMaterial {
      unlit: true,
      base_color: color,
      ..default()
    };
    let material = world
      .resource_mut::<Assets<StandardMaterial>>()
      .add(material);

    world
     .entity_mut(entity_id)
     .insert((
       PbrBundle {
          mesh,
          material,
          transform: Transform::from_xyz(spawn_point.x, spawn_point.y, spawn_point.z),
          ..Default::default()
       },
     ))
     .insert(PlayerInput::default())
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
