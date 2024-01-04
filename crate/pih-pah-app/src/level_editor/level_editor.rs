use bevy::prelude::*;

use crate::{
    controls::Action,
    game::GameState,
    lobby::{Lobby, PlayerId},
};

// TODO: better naming
#[derive(Debug, Default, Component)]
pub struct EditorNavigator;

pub struct LevelEditorPlugins;

impl Plugin for LevelEditorPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, navigation.run_if(in_state(GameState::LevelEditor)));
    }
}

fn navigation(
    lobby: ResMut<Lobby>,
    mut query: Query<(&PlayerId, &mut Transform), With<EditorNavigator>>,
) {
    for (player_id, mut transform) in query.iter_mut() {
        if let Some(player) = lobby.players.get(player_id) {
            let dx = (player.inputs.get(Action::LeverEditorForward).as_boolean() as i8
                - player.inputs.get(Action::LevelEditorBackward).as_boolean() as i8)
                as f32;
            let dy = (player.inputs.get(Action::LevelEditorRight).as_boolean() as i8
                - player.inputs.get(Action::LevelEditorLeft).as_boolean() as i8)
                as f32;

            transform.translation.x += dx;
            transform.translation.y += dy;
        } else {
            // TODO: global error handler like in `bevy_renet`
            log::error!(
                "Player {:?} not found in lobby, but exist in world",
                player_id
            )
        }
    }
}
