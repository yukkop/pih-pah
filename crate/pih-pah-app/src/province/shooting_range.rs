use bevy::prelude::*;
use crate::util::ResourceAction;
use crate::world::PromisedScene;

const PRIMARY_CAMERA_ORDER: isize = 3;

#[derive(Component)]
struct Affiliation;

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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5., 2.5,  5.).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            order: PRIMARY_CAMERA_ORDER,
            ..default()
        },
        ..Default::default()
    }).insert(Affiliation);

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
