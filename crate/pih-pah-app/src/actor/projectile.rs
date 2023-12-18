use bevy::prelude::*;
use bevy_xpbd_3d::components::{LinearVelocity, Mass, RigidBody, Collider, GravityDirection};
use serde::{Serialize, Deserialize};
use bevy::ecs::system::EntityCommands;

use crate::{
    component::{Despawn, DespawnReason, AxisName},
    extend_commands, world::{ProjectileIdSeq, LinkId}, lobby::host::SpawnProjectileEvent,
};

use super::{Trace, physics_bundle::PhysicsBundle, Actor};

#[derive(Default, Serialize, Deserialize)]
pub struct Projectile{
    pub position: Vec3,
    pub direction: Vec3,
    pub power: f32,
    pub mass: f32,
    pub color: Color,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectileShell{
    pub color: Color,
    pub id: LinkId,
}

extend_commands!(
    spawn_projectile(projectile: Projectile),
    |world: &mut World, entity_id: Entity, projectile: Projectile| {
        let size = 0.5;
        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Mesh::try_from(shape::Cube { size }).unwrap());
        let material = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                base_color: projectile.color,
                ..default()
            });

        let link_id = world.resource_mut::<ProjectileIdSeq>().shift();

        world.entity_mut(entity_id).insert((
            PbrBundle {
                mesh: mesh,
                material: material,
                transform: Transform::from_translation(projectile.position),
                ..default()
            },
            PhysicsBundle::from_rigid_body(RigidBody::Dynamic),
            Collider::cuboid(size, size, size),
            Despawn::new((
                // DespawnReason::More(200., AxisName::Y),
                DespawnReason::Less(-10., AxisName::Y),
                DespawnReason::More(100., AxisName::X),
                DespawnReason::Less(-100., AxisName::X),
                DespawnReason::More(100., AxisName::Z),
                DespawnReason::Less(-100., AxisName::Z),
            )),
            Trace::new(0.2, 0.005, projectile.color),
            LinearVelocity::from(projectile.direction * projectile.power),
            Mass(projectile.mass),
            GravityDirection::new(Vec3::Y * -0.2),
            Actor,
            link_id.clone(),
        ));

        world.send_event(SpawnProjectileEvent(link_id));
    }
);

extend_commands!(
    spawn_projectile_shell(projectile: ProjectileShell),
    |world: &mut World, entity_id: Entity, projectile: ProjectileShell| {
        let size = 0.3;
        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Mesh::try_from(shape::Cube { size }).unwrap());
        let material = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                base_color: projectile.color,
                ..default()
            });

        world.entity_mut(entity_id).insert((
            PbrBundle {
                mesh: mesh,
                material: material,
                ..default()
            },
            Trace::new(0.5, 0.05, projectile.color),
            projectile.id,
            Actor,
        ));
    }
);