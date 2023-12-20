use crate::actor::UnloadActorsEvent;
use crate::character::{spawn_character, spawn_tied_camera, TiedCamera};
use crate::component::{DespawnReason, Respawn};
use crate::lobby::host::generate_player_color;
use crate::lobby::LobbyState;
use crate::map::{is_loaded, MapState, SpawnPoint};
use crate::world::Me;
use bevy::app::{App, Plugin, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::event::{EventReader, Events, EventWriter};
use bevy::ecs::query::With;
use bevy::ecs::schedule::{Condition, NextState, OnExit};
use bevy::ecs::system::{Query, Res, ResMut};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{in_state, Commands, IntoSystemConfigs, OnEnter};
use log::info;

use super::{ChangeMapLobbyEvent, MapLoaderState, PlayerId, Inputs, PlayerInputs, Character};

pub struct SingleLobbyPlugins;

impl Plugin for SingleLobbyPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LobbyState::Single), setup)
            .add_systems(
                Update,
                load_processing
                    .run_if(in_state(LobbyState::Single).and_then(in_state(MapLoaderState::No))),
            )
            .add_systems(Update, change_map.run_if(in_state(LobbyState::Single)))
            .add_systems(OnExit(LobbyState::Single), teardown);
    }
}

fn setup(mut map_events: ResMut<Events<ChangeMapLobbyEvent>>) {
    map_events.send(ChangeMapLobbyEvent(MapState::ShootingRange));
}

pub fn load_processing(
    mut commands: Commands,
    spawn_point: Res<SpawnPoint>,
    mut query: Query<&mut Respawn, With<Me>>,
    mut next_state_map: ResMut<NextState<MapLoaderState>>,
) {
    info!("LoadProcessing: {:#?}", spawn_point);
    if is_loaded(&spawn_point) {
        match query.get_single_mut() {
            Err(_) => {
                // spawn character fitst time
                let random_i32 = rand::random::<i32>();
                let color = generate_player_color(random_i32 as u32);

                let player_entity = commands
                    .spawn_character(PlayerId::HostOrSingle, color, spawn_point.random_point())
                    .insert(Me)
                    .id();
                commands.spawn_tied_camera(player_entity);
            }
            Ok(mut respawn) => {
                // respawn character
                respawn.replase_spawn_point(spawn_point.clone());
                respawn.insert_reason(DespawnReason::Forced);
            }
        }
        next_state_map.set(MapLoaderState::Yes);
    }
}

pub fn change_map(
    mut change_map_event: EventReader<ChangeMapLobbyEvent>,
    mut next_state_map: ResMut<NextState<MapState>>,
    mut unload_actors_event: EventWriter<UnloadActorsEvent>,
) {
    for ChangeMapLobbyEvent(state) in change_map_event.read() {
        next_state_map.set(*state);

        unload_actors_event.send(UnloadActorsEvent);
    }
}

fn teardown(
    mut commands: Commands,
    tied_camera_query: Query<Entity, With<TiedCamera>>,
    char_query: Query<Entity, With<Character>>,
    mut unload_actors_event: EventWriter<UnloadActorsEvent>,
) {
    if let Ok(entity) = tied_camera_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
    if let Ok(entity) = char_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    unload_actors_event.send(UnloadActorsEvent);
}
