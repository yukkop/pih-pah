use crate::world::PromisedScene;
use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

use super::ProvinceState;

#[derive(Component)]
struct Affiliation;

pub struct ShootingRangePlugins;

impl Plugin for ShootingRangePlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ProvinceState::ShootingRange), load)
            .add_systems(OnExit(ProvinceState::ShootingRange), unload);
    }
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::WHITE,
                illuminance: 4000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 200.0, 0.0),
                rotation: Quat::from_rotation_x((-1.0_f32).atan()),
                ..default()
            },
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 4.0,
                ..default()
            }
            .into(),
            ..default()
        })
        .insert(Affiliation);

    let scene = asset_server.load("test_province.glb#Scene0");

    commands
        .spawn(SceneBundle { scene, ..default() })
        .insert(PromisedScene)
        .insert(Affiliation);
}

fn unload(mut commands: Commands, affiliation_query: Query<Entity, With<Affiliation>>) {
    for entity in affiliation_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
