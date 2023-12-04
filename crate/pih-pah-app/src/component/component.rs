use bevy::app::{App, PreUpdate};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::log::info;
use bevy::prelude::{Component, Plugin, Vec3};
use bevy::transform::components::{GlobalTransform, Transform};
use bevy_xpbd_3d::components::{AngularVelocity, LinearVelocity};
use log::warn;

use super::despawn_type::{DespawnReason, IntoDespawnTypeVec};

#[derive(Component)]
pub struct Respawn {
    reason: Vec<DespawnReason>,
    spawn_point: Vec3,
}

impl Respawn {
    pub fn from_vec3(spawn_point: Vec3) -> Self {
        Self { reason: vec![],  spawn_point }
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
    for (mut respawn, mut transform, global_transform, entity) in respawn_query.iter_mut() {
        if respawn.reason.contains(&DespawnReason::Force) {
            transform.translation = respawn.spawn_point;
            if let Ok((mut linear_velocity, mut angular_velocity)) = velocity_query.get_mut(entity) {
                linear_velocity.0 = Vec3::ZERO;
                angular_velocity.0 = Vec3::ZERO;
            }
            respawn.reason.retain(|reason| reason != &DespawnReason::Force);
        } else if global_transform.translation().y < -10.0 {
            transform.translation = respawn.spawn_point;
            if let Ok((mut linear_velocity, mut angular_velocity)) = velocity_query.get_mut(entity)
            {
                linear_velocity.0 = Vec3::ZERO;
                angular_velocity.0 = Vec3::ZERO;
            }
        }
    }
}

fn despawn() {}
