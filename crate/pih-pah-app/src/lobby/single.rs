use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{Color, Commands, in_state, IntoSystemConfigs, OnEnter};
use renet::ClientId;
use crate::character::{spawn_character, spawn_tied_camera};
use crate::lobby::LobbyState;
use crate::world::Me;

pub struct SingleLobbyPlugins;

impl Plugin for SingleLobbyPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LobbyState::Single),
                        setup);
        app.add_systems(Update,
                        update.run_if(in_state(LobbyState::Single)));
    }
}

fn setup(
    mut commands: Commands
) {
    let a = Vec3::new(0., 10., 0.);
    let entity = commands
        .spawn_character(ClientId::from_raw(0), Color::RED, a).insert(Me).id();
    commands.spawn_tied_camera(entity);
}

fn update() {

}