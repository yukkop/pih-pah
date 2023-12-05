use bevy::app::{App, PreUpdate, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::{Query, Res, Commands};
use bevy::log::info;
use bevy::prelude::{Component, Plugin, Vec3, DerefMut, Deref};
use bevy::time::{Timer, Time};
use bevy::transform::components::{GlobalTransform, Transform};
use bevy_xpbd_3d::components::{AngularVelocity, LinearVelocity, CollisionLayers};

use crate::component::AxisName;
use crate::world::MyLayers;

use super::despawn_type::{DespawnReason, IntoDespawnTypeVec};

#[derive(Component)]
pub struct Respawn {
    reason: Vec<DespawnReason>,
    spawn_point: Vec3,
    untuched_on_spawn: UntouchedTimerValue,
}

#[derive(PartialEq, Debug)]
pub enum UntouchedTimerValue {
    None,
    Timer(f32),
}

#[derive(Deref, DerefMut, Component)]
pub struct  UntouchedTimer(Timer);

impl Respawn {
    pub fn new<T: IntoDespawnTypeVec>(reason: T, spawn_point: Vec3, untuched_on_spawn: UntouchedTimerValue) -> Self {
        Self { reason: reason.into_despawn_type_vec(),  spawn_point, untuched_on_spawn }
    }

    pub fn from_vec3(spawn_point: Vec3) -> Self {
        Self { reason: vec![],  spawn_point, untuched_on_spawn: UntouchedTimerValue::None }
    }

    pub fn insert_reason(&mut self, reason: DespawnReason) {
        self.reason.push(reason);
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
        app
            .add_systems(PreUpdate, (respawn, despawn))
            .add_systems(Update, timer_tick_system);
    }
}

fn timer_tick_system(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut UntouchedTimer)>) {
    for (entity, mut timer) in query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity)
                .insert(collisionlayers::new([mylayers::default], [mylayers::default, mylayers::untouched]))
                .remove::<UntouchedTimer>();
        }
    }
}

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
        if let UntouchedTimerValue::Timer(val) = respawn.untuched_on_spawn {
            commands
                .entity(entity).insert(
                    UntouchedTimer(Timer::from_seconds(val, bevy::time::TimerMode::Once)))
                .insert(CollisionLayers::new([MyLayers::ActorNoclip], [MyLayers::Default]));
        }
        transform.translation = respawn.spawn_point;
        if let Ok((mut linear_velocity, mut angular_velocity)) = velocity_query.get_mut(entity) {
            linear_velocity.0 = Vec3::ZERO;
            angular_velocity.0 = Vec3::ZERO;
        }
    }

    for (mut respawn, mut transform, global_transform, entity) in respawn_query.iter_mut() {
        for reason in respawn.reason.clone() {
            match reason {
                DespawnReason::Forced => {
                    respawn_act(&mut commands, &mut respawn, &mut transform,  entity, &mut velocity_query);
                    respawn.reason.retain(|reason| reason != &DespawnReason::Forced);
                },
                DespawnReason::Less(val, axis) => {
                    match axis {
                        AxisName::X => {
                            if global_transform.translation().x < val {
                                respawn_act(&mut commands, &mut respawn,&mut transform, entity, &mut velocity_query);
                            }
                        }, 
                        AxisName::Y => {
                            if global_transform.translation().y < val {
                                respawn_act(&mut commands, &mut respawn,&mut transform, entity, &mut velocity_query);
                            }
                        }, 
                        AxisName::Z => {
                            if global_transform.translation().z < val {
                                respawn_act(&mut commands, &mut respawn,&mut transform, entity, &mut velocity_query);
                            }
                        }, 
                    }

                },
                DespawnReason::More(val, axis) => {
                    match axis {
                        AxisName::X => {
                            if global_transform.translation().x > val {
                                respawn_act(&mut commands, &mut respawn,&mut transform, entity, &mut velocity_query);
                            }
                        }, 
                        AxisName::Y => {
                            if global_transform.translation().y > val {
                                respawn_act(&mut commands, &mut respawn,&mut transform, entity, &mut velocity_query);
                            }
                        }, 
                        AxisName::Z => {
                            if global_transform.translation().z > val {
                                respawn_act(&mut commands, &mut respawn,&mut transform, entity, &mut velocity_query);
                            }
                        }, 
                    }
                },
            }
        }
    }
}

fn despawn() {}
