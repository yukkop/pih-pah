use bevy::app::{App, PreUpdate};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::prelude::{Component, Plugin, Vec3};
use bevy::transform::components::{GlobalTransform, Transform};
use bevy_xpbd_3d::components::{AngularVelocity, LinearVelocity};

use crate::component::AxisName;

use super::despawn_type::{DespawnReason, IntoDespawnTypeVec};

#[derive(Component)]
pub struct Respawn {
    reason: Vec<DespawnReason>,
    spawn_point: Vec3,
    untuched_on_spawn: UntouchedTimer,
}

enum UntouchedTimer {
    None,
    Timer(f32),
}


impl Respawn {
    pub fn new<T: IntoDespawnTypeVec>(reason: T, spawn_point: Vec3) -> Self {
        Self { reason: reason.into_despawn_type_vec(),  spawn_point, untuched_on_spawn: UntouchedTimer::None }
    }

    pub fn from_vec3(spawn_point: Vec3) -> Self {
        Self { reason: vec![],  spawn_point, untuched_on_spawn: UntouchedTimer::None }
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
        app.add_systems(PreUpdate, (respawn, despawn));
    }
}

fn respawn(
    mut respawn_query: Query<(&mut Respawn, &mut Transform, &GlobalTransform, Entity)>,
    mut velocity_query: Query<(&mut LinearVelocity, &mut AngularVelocity), With<Respawn>>,
) {
    fn respawn_act(
        respawn: &mut Respawn,
        transform: &mut Transform,
        entity: Entity,
        velocity_query: &mut Query<(&mut LinearVelocity, &mut AngularVelocity), With<Respawn>>,
    ) {
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
                    respawn_act(&mut respawn, &mut transform,  entity, &mut velocity_query);
                },
                DespawnReason::Less(val, axis) => {
                    match axis {
                        AxisName::X => {
                            if global_transform.translation().x < val {
                                respawn_act(&mut respawn,&mut transform, entity, &mut velocity_query);
                            }
                        }, 
                        AxisName::Y => {
                            if global_transform.translation().y < val {
                                respawn_act(&mut respawn,&mut transform, entity, &mut velocity_query);
                            }
                        }, 
                        AxisName::Z => {
                            if global_transform.translation().z < val {
                                respawn_act(&mut respawn,&mut transform, entity, &mut velocity_query);
                            }
                        }, 
                    }

                },
                DespawnReason::More(val, axis) => {
                    match axis {
                        AxisName::X => {
                            if global_transform.translation().x > val {
                                respawn_act(&mut respawn,&mut transform, entity, &mut velocity_query);
                            }
                        }, 
                        AxisName::Y => {
                            if global_transform.translation().y > val {
                                respawn_act(&mut respawn,&mut transform, entity, &mut velocity_query);
                            }
                        }, 
                        AxisName::Z => {
                            if global_transform.translation().z > val {
                                respawn_act(&mut respawn,&mut transform, entity, &mut velocity_query);
                            }
                        }, 
                    }
                },
            }
        }
    }
}

fn despawn() {}
