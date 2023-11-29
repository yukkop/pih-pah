use bevy::prelude::*;
use crate::province;

pub struct WorldPlugins;

impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    commands: Commands,
    materials: ResMut<Assets<StandardMaterial>>,
    mesh: ResMut<Assets<Mesh>>
) {
    let (_command, _mesh, _material) = province::menu::load(commands, mesh, materials);
}