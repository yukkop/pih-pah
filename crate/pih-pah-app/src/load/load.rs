use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        event::{Event, EventReader},
        schedule::{common_conditions::in_state, IntoSystemConfigs, NextState, States},
        system::{Commands, Res, ResMut, Resource},
    },
    log::info,
};

use crate::{lobby::LobbyState, province::SpawnPoint};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum LoadState {
    #[default]
    None = 0,
    Load = 1,
}

#[derive(Resource)]
pub struct LoadResource(LobbyState);

#[derive(Event)]
pub struct LoadEvent(pub LobbyState);

pub struct LoadPlugins;

impl Plugin for LoadPlugins {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadEvent>()
            .add_state::<LoadState>()
            .add_systems(Update, load_event)
            .add_systems(Update, load_processing.run_if(in_state(LoadState::Load)));
    }
}

fn load_event(
    mut commands: Commands,
    mut event: EventReader<LoadEvent>,
    mut next_state_load: ResMut<NextState<LoadState>>,
) {
    for LoadEvent(state) in event.read() {
        info!("LoadEvent: {:?}", state);
        commands.insert_resource(LoadResource(*state));
        next_state_load.set(LoadState::Load);
    }
}

fn load_processing(
    mut commands: Commands,
    res: Res<LoadResource>,
    // check scene is load by spawn point
    spawn_point: Res<SpawnPoint>,
    mut next_state_lobby: ResMut<NextState<LobbyState>>,
    mut next_state_load: ResMut<NextState<LoadState>>,
) {
    if !spawn_point.is_empty() {
        next_state_lobby.set(res.0);
        commands.remove_resource::<LoadResource>();
        next_state_load.set(LoadState::None);
    }
}
