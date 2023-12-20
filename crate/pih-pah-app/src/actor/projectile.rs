use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_xpbd_3d::components::{
    Collider, GravityDirection, LinearVelocity, MassPropertiesBundle, RigidBody,
};
use serde::{Deserialize, Serialize};

use crate::{
    component::{AxisName, Despawn, DespawnReason},
    extend_commands,
    lobby::host::SpawnProjectileEvent,
    world::{LinkId, ProjectileIdSeq},
};

use super::{physics_bundle::PhysicsBundle, Actor, TransformOptimalTrace};

#[derive(Default, Serialize, Deserialize)]
pub struct Projectile {
    pub position: Vec3,
    pub direction: Vec3,
    pub power: f32,
    pub mass: f32,
    pub color: Color,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectileShell {
    pub color: Color,
    pub id: LinkId,
}

const SIZE: f32 = 0.5;

extend_commands!(
    spawn_projectile(projectile: Projectile),
    |world: &mut World, entity_id: Entity, projectile: Projectile| {
        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Mesh::try_from(shape::Cube { size: SIZE }).unwrap());
        let material = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                base_color: projectile.color,
                ..default()
            });

        let link_id = world.resource_mut::<ProjectileIdSeq>().shift();

        world.entity_mut(entity_id).insert((
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(projectile.position),
                ..default()
            },
            Despawn::new((
                DespawnReason::More(200., AxisName::Y),
                DespawnReason::Less(-10., AxisName::Y),
                DespawnReason::More(100., AxisName::X),
                DespawnReason::Less(-100., AxisName::X),
                DespawnReason::More(100., AxisName::Z),
                DespawnReason::Less(-100., AxisName::Z),
            )),
            // PhysicsOptimalTrace::new(0.2, 0.005, projectile.color, SIZE / 2.),
            GravityDirection::new(Vec3::Y * -0.2),
            Actor,
            link_id.clone(),
        ))
        .insert((
            PhysicsBundle::from_rigid_body(RigidBody::Dynamic),
            Collider::cuboid(SIZE, SIZE, SIZE),
            MassPropertiesBundle::default(),
            LinearVelocity::from(projectile.direction * projectile.power)));

        world.send_event(SpawnProjectileEvent(link_id, projectile.color));
    }
);

extend_commands!(
    spawn_projectile_shell(projectile: ProjectileShell),
    |world: &mut World, entity_id: Entity, projectile: ProjectileShell| {
        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Mesh::try_from(shape::Cube { size: SIZE }).unwrap());
        let material = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                base_color: projectile.color,
                ..default()
            });

        world.entity_mut(entity_id).insert((
            PbrBundle {
                mesh,
                material,
                ..default()
            },
            // Trace::new(0.5, 0.05, projectile.color),
            projectile.id,
            Actor,
            TransformOptimalTrace::new(0.2, 0.005, projectile.color, SIZE / 2.),
        ));
    }
);
