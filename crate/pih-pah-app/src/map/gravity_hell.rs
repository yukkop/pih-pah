use crate::{lobby::MapLoaderState, world::PromisedScene};
use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

use super::{spawn_point::SpawnPoint, MapState};

#[derive(Component)]
struct Affiliation;

pub struct GravityHellPlugins;

impl Plugin for GravityHellPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MapState::GravityHell), load)
            .add_systems(OnExit(MapState::GravityHell), unload);
    }
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 4000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 200.0, 0.0),
            rotation: Quat::from_rotation_x(0.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            ..default()
        }
        .into(),
        ..default()
    });

    let scene = asset_server.load("gravity_hell.glb#Scene0");

    commands.spawn((
        SceneBundle { scene, ..default() },
        PromisedScene,
        Affiliation,
        Name::new("GravityHell"),
    ));

    commands.insert_resource(SpawnPoint::new(Vec3::new(0., 5., 0.)));
}

fn unload(
    mut commands: Commands,
    affiliation_query: Query<Entity, With<Affiliation>>,
    mut next_state_map: ResMut<NextState<MapLoaderState>>,
) {
    for entity in affiliation_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.insert_resource(SpawnPoint::empty());
    next_state_map.set(MapLoaderState::No);
}
