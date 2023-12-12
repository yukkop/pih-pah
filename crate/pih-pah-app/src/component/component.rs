use bevy::app::{App, PreUpdate, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::log::info;
use bevy::prelude::{Component, Deref, DerefMut, Plugin, Vec3};
use bevy::time::{Time, Timer};
use bevy::transform::components::{GlobalTransform, Transform};
use bevy_xpbd_3d::components::{AngularVelocity, CollisionLayers, LinearVelocity};

use crate::component::AxisName;
use crate::province::SpawnPoint;
use crate::world::CollisionLayer;

use super::despawn_type::{DespawnReason, IntoDespawnTypeVec};

/// A component representing respawn behavior for an entity.
///
/// The [`Respawn`] component is used to control how an entity respawns in a game. It includes information about the respawn reasons,
/// the spawn point, and a timer value for keeping the entity untouched upon spawn.
#[derive(Component)]
pub struct Respawn {
    /// Reasons for respawning.
    reason: Vec<DespawnReason>,
    /// The spawn point for the entity.
    spawn_point: SpawnPoint,
    /// Duration for keeping the [`CollisionLayers`] into [`noclip`](CollisionLayer::ActorNoclip) [`CollisionLayer`] upon spawn.
    noclip: NoclipDuration,
}

/// An enumeration representing the duration of time an actor will remain [`noclip`](CollisionLayer::ActorNoclip).
///
/// The [`NoclipDuration`] enum is used to specify how long an actor should remain [`noclip`](CollisionLayer::ActorNoclip) before some action or event takes place.
#[derive(PartialEq, Debug)]
pub enum NoclipDuration {
    /// Indicates that there is no [`noclip`](CollisionLayer::ActorNoclip) duration, and the actor can be acted upon immediately.
    None,
    /// Specifies a timed duration in seconds before the actor can be acted upon.
    Timer(f32),
}

/// A component representing a timer for a [`noclip`](CollisionLayer::ActorNoclip) mode.
///
/// The [`NoclipTimer`] component is used to manage the duration of a [`noclip`](CollisionLayer::ActorNoclip) mode in a game.
/// It wraps a [`Timer`] for time tracking and management.
#[derive(Deref, DerefMut, Component)]
pub struct NoclipTimer(Timer);

impl Respawn {
    pub fn new<T: IntoDespawnTypeVec>(
        reason: T,
        spawn_point: SpawnPoint,
        untouched_on_spawn: NoclipDuration,
    ) -> Self {
        Self {
            reason: reason.into_despawn_type_vec(),
            spawn_point,
            noclip: untouched_on_spawn,
        }
    }

    pub fn from_vec3(spawn_point: Vec3) -> Self {
        Self {
            reason: vec![],
            spawn_point: SpawnPoint::new(spawn_point),
            noclip: NoclipDuration::None,
        }
    }

    pub fn insert_reason(&mut self, reason: DespawnReason) {
        self.reason.push(reason);
    }

    pub fn clear_spawn_point(&mut self) {
        self.spawn_point = SpawnPoint::default();
    }

    pub fn replase_spawn_point(&mut self, spawn_point: SpawnPoint) {
        self.spawn_point = spawn_point;
    }
}

#[derive(Debug)]
struct Despawn(Vec<DespawnReason>);

impl Despawn {
    // TODO
    #[allow(dead_code)]
    pub fn new<T: IntoDespawnTypeVec>(types: T) -> Self {
        Self(types.into_despawn_type_vec())
    }
}

pub struct ComponentPlugins;

impl Plugin for ComponentPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (respawn, despawn))
            .add_systems(Update, noclip_timer);
    }
}

/// Updates entities with a [`NoclipTimer`] component to toggle [`noclip`](CollisionLayer::ActorNoclip) mode temporarily.
///
/// The `noclip_timer` function iterates through entities with a [`NoclipTimer`] component and checks if the timer has finished.
/// If the timer has finished, it adds a specific collision layer to the entity, indicating [`noclip`](CollisionLayer::ActorNoclip) mode, and removes the [`NoclipTimer`] component.
fn noclip_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut NoclipTimer)>,
) {
    for (entity, mut timer) in query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands
                .entity(entity)
                .insert(CollisionLayers::new(
                    [CollisionLayer::Default],
                    [CollisionLayer::Default, CollisionLayer::ActorNoclip],
                ))
                .remove::<NoclipTimer>();
        }
    }
}

/// Processes a [`Entity`] with [`Respawn`] [`Component`]
///
/// Move actors on respawn position and optionally rest [`LinearVelocity`] and [`AngularVelocity`]
/// if one of `reason` ([`DespawnReason`]) is true
fn respawn(
    mut commands: Commands,
    mut respawn_query: Query<(&mut Respawn, &mut Transform, &GlobalTransform, Entity)>,
    mut velocity_query: Query<(&mut LinearVelocity, &mut AngularVelocity), With<Respawn>>,
) {
    fn respawn_act(
        commands: &mut Commands,
        respawn: &mut Respawn,
        transform: &mut Transform,
        entity: Entity,
        velocity_query: &mut Query<(&mut LinearVelocity, &mut AngularVelocity), With<Respawn>>,
    ) {
        info!("Respawn entity: {:?}", entity);
        info!("Respawn cords: {:?}", respawn.spawn_point);
        if let NoclipDuration::Timer(val) = respawn.noclip {
            commands
                .entity(entity)
                .insert(NoclipTimer(Timer::from_seconds(
                    val,
                    bevy::time::TimerMode::Once,
                )))
                .insert(CollisionLayers::new(
                    [CollisionLayer::ActorNoclip],
                    [CollisionLayer::Default],
                ));
        }
        transform.translation = respawn.spawn_point.random_point();
        if let Ok((mut linear_velocity, mut angular_velocity)) = velocity_query.get_mut(entity) {
            linear_velocity.0 = Vec3::ZERO;
            angular_velocity.0 = Vec3::ZERO;
        }
    }

    for (mut respawn, mut transform, global_transform, entity) in respawn_query.iter_mut() {
        for reason in respawn.reason.clone() {
            match reason {
                DespawnReason::Forced => {
                    respawn_act(
                        &mut commands,
                        &mut respawn,
                        &mut transform,
                        entity,
                        &mut velocity_query,
                    );
                    respawn
                        .reason
                        .retain(|reason| reason != &DespawnReason::Forced);
                }
                DespawnReason::Less(val, axis) => match axis {
                    AxisName::X => {
                        if global_transform.translation().x < val {
                            respawn_act(
                                &mut commands,
                                &mut respawn,
                                &mut transform,
                                entity,
                                &mut velocity_query,
                            );
                        }
                    }
                    AxisName::Y => {
                        if global_transform.translation().y < val {
                            respawn_act(
                                &mut commands,
                                &mut respawn,
                                &mut transform,
                                entity,
                                &mut velocity_query,
                            );
                        }
                    }
                    AxisName::Z => {
                        if global_transform.translation().z < val {
                            respawn_act(
                                &mut commands,
                                &mut respawn,
                                &mut transform,
                                entity,
                                &mut velocity_query,
                            );
                        }
                    }
                },
                DespawnReason::More(val, axis) => match axis {
                    AxisName::X => {
                        if global_transform.translation().x > val {
                            respawn_act(
                                &mut commands,
                                &mut respawn,
                                &mut transform,
                                entity,
                                &mut velocity_query,
                            );
                        }
                    }
                    AxisName::Y => {
                        if global_transform.translation().y > val {
                            respawn_act(
                                &mut commands,
                                &mut respawn,
                                &mut transform,
                                entity,
                                &mut velocity_query,
                            );
                        }
                    }
                    AxisName::Z => {
                        if global_transform.translation().z > val {
                            respawn_act(
                                &mut commands,
                                &mut respawn,
                                &mut transform,
                                entity,
                                &mut velocity_query,
                            );
                        }
                    }
                },
            }
        }
    }
}

fn despawn() {}
