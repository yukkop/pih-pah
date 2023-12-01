use bevy::prelude::*;
use crate::util::ResourceAction;
use crate::world::PromisedScene;

#[derive(Component)]
pub struct Affiliation;


#[derive(Event)]
pub struct ShootingRangeEvent(pub ResourceAction);

pub struct ShootingRangePlugins;

impl Plugin for ShootingRangePlugins {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShootingRangeEvent>()
            .add_systems(Update, handle_action);
    }
}

fn handle_action(
    mut reader: EventReader<ShootingRangeEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    affiliation_query: Query<Entity, With<Affiliation>>,
) {
    for ShootingRangeEvent(action) in reader.read() {
        match action {
            ResourceAction::Load => {
                load(&mut commands, &asset_server);
            },
            ResourceAction::Unload => {
                unload(&mut commands, &affiliation_query);
            },
        }
    }
}

fn load(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
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
    commands: &mut Commands,
    affiliation_query: &Query<Entity, With<Affiliation>>,
) {
    for entity in affiliation_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
