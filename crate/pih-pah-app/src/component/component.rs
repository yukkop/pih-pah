use bevy::app::{App, Update};
use bevy::prelude::{Component, Plugin, Vec3};

#[derive(Component)]
pub struct Respawn(Vec3);
#[derive(Component)]
pub struct Despawn();

enum DespawnType {
    Abyss
}

pub struct ComponentPlugins;

impl Plugin for ComponentPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (respawn, despawn));
    }
}

fn respawn() {

}

// fn character_respawn(
//     mut query: Query<(&mut Position, &mut LinearVelocity, &Character)>,
// ) {
//     for (mut position, mut linear_velocity, _player) in query.iter_mut() {
//         if position.y < -5. {
//             position.x = PLAYER_SPAWN_POINT.x;
//             position.y = PLAYER_SPAWN_POINT.y;
//             position.z = PLAYER_SPAWN_POINT.z;
//
//             linear_velocity.z = 0.;
//             linear_velocity.y = 0.;
//             linear_velocity.x = 0.;
//         }
//     }
// }

fn despawn() {

}