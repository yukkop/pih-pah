use bevy::prelude::*;

use crate::{game::GameState, lobby::Lobby, controls::Action};

pub struct EditorNavigator;

pub struct LevelEditorPlugins;

impl Plugin for LevelEditorPlugins {
    fn build(&self, app: &mut App) {
        app
           .add_systems(Update, navigation.run_if(in_state(GameState::LevelEditor)));
    }
}

fn navigation(
    lobby: ResMut<Lobby>,
) {

    for (_player_id, players) in lobby.players.iter() {
        let _dx = (players.inputs.get(Action::LeverEditorForward).as_boolean() as i8 - players.inputs.get(Action::LevelEditorBackward).as_boolean() as i8) as f32;
        let _dy = (players.inputs.get(Action::LvelEditorRight).as_boolean() as i8 - players.inputs.get(Action::LevelEditorLeft).as_boolean() as i8) as f32;
        if players.inputs.get(Action::LevelEditorLeft).into() {
            
        }
    }
}