use bevy::prelude::*;

const PRIMARY_CAMERA_ORDER: isize = 3;

pub struct MenuPlugins;

impl Plugin for MenuPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup() {

}

pub fn load<'a>(
    mut commands: Commands<'a, 'a>,
    mut mesh: ResMut<'a, Assets<Mesh>>,
    mut materials: ResMut<'a, Assets<StandardMaterial>>,
) -> (Commands<'a, 'a>, ResMut<'a, Assets<Mesh>>, ResMut<'a, Assets<StandardMaterial>>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-5., 2.5,  5.).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            order: PRIMARY_CAMERA_ORDER,
            ..default()
        },
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0., 4.0, 0.),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: mesh.add(Mesh::from(shape::Plane::from_size(5.0))),
            material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        Name::new("Terrain"),
    ));

    (commands, mesh, materials)
}

fn unload() {

}