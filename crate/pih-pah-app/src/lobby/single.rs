use crate::character::{spawn_character, spawn_tied_camera, TiedCamera};
use crate::component::{Respawn, DespawnReason};
use crate::lobby::LobbyState;
use crate::lobby::host::generate_player_color;
use crate::province::{SpawnPoint, is_loaded, ProvinceState};
use crate::world::Me;
use bevy::app::{App, Plugin, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::event::{Events, EventReader};
use bevy::ecs::query::With;
use bevy::ecs::schedule::{OnExit, NextState, Condition};
use bevy::ecs::system::{Query, Res, ResMut};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{in_state, Commands, IntoSystemConfigs, OnEnter};
use log::info;

use super::{PlayerId, PlayerInput, MapLoader, ChangeProvinceLobbyEvent};

pub struct SingleLobbyPlugins;

impl Plugin for SingleLobbyPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(LobbyState::Single), setup)
            .add_systems(Update, load_processing.run_if(in_state(LobbyState::Single).and_then(in_state(MapLoader::No))))
            .add_systems(Update, send_change_province.run_if(in_state(LobbyState::Single)))
            .add_systems(OnExit(LobbyState::Single), teardown);
    }
}

fn setup(
    mut province_events: ResMut<Events<ChangeProvinceLobbyEvent>>,
) {
    province_events.send(ChangeProvinceLobbyEvent(ProvinceState::ShootingRange));
}

pub fn load_processing(
    mut commands: Commands,
    spawn_point: Res<SpawnPoint>,
    mut query: Query<&mut Respawn, With<Me>>,
    mut next_state_map: ResMut<NextState<MapLoader>>,
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
            },
            Ok(mut respawn) => {
                // respawn character
                respawn.replase_spawn_point(spawn_point.clone());
                respawn.insert_reason(DespawnReason::Forced);
            }
        }
        next_state_map.set(MapLoader::Yes);
    }
}

pub fn send_change_province(
    mut change_province_event: EventReader<ChangeProvinceLobbyEvent>,
    mut next_state_province: ResMut<NextState<ProvinceState>>,
) {
    for ChangeProvinceLobbyEvent(state) in change_province_event.read() {
        next_state_province.set(*state);
    }
}

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
