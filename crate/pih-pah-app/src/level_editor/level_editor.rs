use bevy::prelude::*;

use crate::game::GameState;

pub struct LevelEditorPlugins;

impl Plugin for LevelEditorPlugins {
    fn build(&self, app: &mut App) {
        app
           .add_systems(Update, movement.run_if(in_state(GameState::LevelEditor)));
    }
}

fn movement(

) {
    todo!("Movement");
}