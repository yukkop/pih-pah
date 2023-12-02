use bevy::prelude::*;
use crate::world::PromisedScene;

use super::ProvinceState;

#[derive(Component)]
struct Affiliation;


pub struct ShootingRangePlugins;

impl Plugin for ShootingRangePlugins {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(ProvinceState::ShootingRange), load)
            .add_systems(OnExit(ProvinceState::ShootingRange), unload);
    }
}

fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 5000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0., 8.0, 0.),
            ..default()
        },
    )).insert(Affiliation);

    let scene = asset_server.load("test_province.glb#Scene0");

    commands.spawn(SceneBundle{
        scene,
        ..default()
    })
    .insert(PromisedScene)
    .insert(Affiliation);
}

fn unload(
    mut commands: Commands,
    affiliation_query: Query<Entity, With<Affiliation>>,
) {
    for entity in affiliation_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
