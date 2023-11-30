use bevy::prelude::*;
use std::f32::consts::PI;
use crate::util::ResourceAction;

const PRIMARY_CAMERA_ORDER: isize = 3;

#[derive(Component)]
struct OrbitLight {
    radius: f32,
    speed: f32,
    angle: f32,
}

#[derive(Event)]
pub struct MenuEvent(pub ResourceAction);

pub struct MenuPlugins;

impl Plugin for MenuPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MenuEvent>()
            .add_systems(Update, handle_action)
            .add_systems(Update, update_light_position);
    }
}

fn handle_action(
    mut reader: EventReader<MenuEvent>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for MenuEvent(action) in reader.read() {
        match action {
            ResourceAction::Load => {
                load(&mut commands, &mut mesh, &mut materials);
            },
            ResourceAction::Unload => {
                unload();
            },
        }
    }
}

fn load(
    commands: &mut Commands,
    mesh: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5., 2.5,  5.).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            order: PRIMARY_CAMERA_ORDER,
            ..default()
        },
        ..Default::default()
    });

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
        OrbitLight {
            radius: 8.0,
            speed: 1.0,
            angle: 0.0,
        },
    ));

    commands.spawn((
        PbrBundle {
            mesh: mesh.add(Mesh::from(shape::Plane::from_size(5.0))),
            material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        Name::new("Terrain"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: mesh.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::GRAY.into()),
            transform: Transform::from_xyz(0., 0.5, 0.),
            ..Default::default()
        },
        Name::new("Cube"),
    ));
}

fn unload() {

}

fn update_light_position(time: Res<Time>, mut query: Query<(&mut OrbitLight, &mut Transform)>) {
    for (mut orbit_light, mut transform) in query.iter_mut() {
        orbit_light.angle += orbit_light.speed * time.delta_seconds();
        if orbit_light.angle > 2.0 * PI {
            orbit_light.angle -= 2.0 * PI;
        }
        transform.translation.x = orbit_light.radius * orbit_light.angle.cos();
        transform.translation.z = orbit_light.radius * orbit_light.angle.sin();
    }
}
