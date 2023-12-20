use bevy::ecs::bundle::Bundle;
use bevy_xpbd_3d::components::{CollisionLayers, Restitution, RigidBody};

use crate::world::CollisionLayer;

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub rigid_body: RigidBody,
    pub restitution: Restitution,
    pub collision_layers: CollisionLayers,
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::default(),
            restitution: Restitution::ZERO,
            collision_layers: CollisionLayers::new(
                [CollisionLayer::Default],
                [CollisionLayer::Default, CollisionLayer::ActorNoclip],
            ),
        }
    }
}

impl PhysicsBundle {
    pub fn from_rigid_body(rigid_body: RigidBody) -> Self {
        Self {
            rigid_body,
            ..Self::default()
        }
    }
}
