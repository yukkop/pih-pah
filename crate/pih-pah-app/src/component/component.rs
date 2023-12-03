use bevy::app::{App, Update, PreUpdate};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::prelude::{Component, Plugin, Vec3};
use bevy::transform::components::{Transform, GlobalTransform};
use bevy_xpbd_3d::components::{AngularVelocity, LinearVelocity};

use super::despawn_type::{IntoDespawnTypeVec, DespawnType};

#[derive(Component)]
pub struct Respawn{
    spawn_point: Vec3,
}

impl Respawn {
    pub fn new(spawn_point: Vec3) -> Self {
        Self {
            spawn_point,
        }
    }
}

#[derive(Debug)]
struct Despawn(Vec<DespawnType>);

impl Despawn {
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
    mut respawn_query: Query<(&Respawn, &mut Transform, &GlobalTransform, Entity)>,
    mut velocity_query: Query<(&mut LinearVelocity, &mut AngularVelocity), With<Respawn>>,
) {
    for (respawn, mut transform, global_transform, entity) in respawn_query.iter_mut() {
        if global_transform.translation().y < -10.0 {
            transform.translation = respawn.spawn_point;
            if let Ok((mut linear_velocity, mut angular_velocity)) = velocity_query.get_mut(entity) {
                linear_velocity.0 = Vec3::ZERO;
                angular_velocity.0 = Vec3::ZERO;
            }
        }
    }
}

fn despawn() {

}