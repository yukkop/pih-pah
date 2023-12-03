use crate::character::{spawn_character, spawn_tied_camera, TiedCamera};
use crate::lobby::LobbyState;
use crate::world::Me;
use bevy::app::{App, Plugin, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::schedule::OnExit;
use bevy::ecs::system::Query;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec3;
use bevy::prelude::{in_state, Color, Commands, IntoSystemConfigs, OnEnter};

use super::{PlayerId, PlayerInput};

pub struct SingleLobbyPlugins;

impl Plugin for SingleLobbyPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LobbyState::Single), setup);
        app.add_systems(Update, update.run_if(in_state(LobbyState::Single)));
        app.add_systems(OnExit(LobbyState::Single), teardown);
    }
}

fn setup(mut commands: Commands) {
    let a = Vec3::new(0., 10., 0.);
    let entity = commands
        .spawn_character(PlayerId::Host, Color::RED, a)
        .insert(Me)
        .id();
    commands.spawn_tied_camera(entity);
}

fn update() {}

fn teardown(
    mut commands: Commands,
    tied_camera_query: Query<Entity, With<TiedCamera>>,
    char_query: Query<Entity, With<PlayerInput>>,
) {
    if let Ok(entity) = tied_camera_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
    if let Ok(entity) = char_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
