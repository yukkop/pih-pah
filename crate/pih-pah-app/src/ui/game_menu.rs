use crate::lobby::LobbyState;
use crate::province::ProvinceState;
use crate::ui::{rich_text, UiAction, TRANSPARENT};
use crate::util::i18n::Uniq::Module;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::UiState;

lazy_static::lazy_static! {
    static ref MODULE: &'static str = module_path!().splitn(3, ':').nth(2).unwrap_or(module_path!());
}

#[derive(Event)]
pub struct GameMenuEvent(pub UiAction);

#[derive(Resource)]
struct State {
    is_active: bool,
}

impl Default for State {
    fn default() -> Self {
        Self { is_active: false }
    }
}

pub struct GameMenuPlugins;

impl Plugin for GameMenuPlugins {
    fn build(&self, app: &mut App) {
        app.add_event::<GameMenuEvent>()
            .init_resource::<State>()
            .add_systems(
                Update,
                (handle_action, handle_state).run_if(in_state(UiState::GameMenu)),
            );
    }
}

fn handle_action(mut reader: EventReader<GameMenuEvent>, mut state: ResMut<State>) {
    for GameMenuEvent(action) in reader.read() {
        match action {
            UiAction::Enable => {
                state.is_active = true;
            }
            UiAction::Disable => {
                state.is_active = false;
            }
            UiAction::Toggle => {
                state.is_active = !state.is_active;
            }
        }
    }
}

fn handle_state(
    mut next_state_lobby: ResMut<NextState<LobbyState>>,
    mut next_state_ui: ResMut<NextState<UiState>>,
    mut next_state_province: ResMut<NextState<ProvinceState>>,
    mut context: EguiContexts,
    state: Res<State>,
    mut ui_game_menu_writer: EventWriter<GameMenuEvent>,
) {
    let ctx = context.ctx_mut();

    let font = egui::FontId {
        family: egui::FontFamily::Monospace,
        ..default()
    };

    if state.is_active {
        egui::Window::new(rich_text("Menu".to_string(), Module(&MODULE), &font))
            .frame(*TRANSPARENT)
            .anchor(egui::Align2::LEFT_BOTTOM, [10., -10.])
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .show(ctx, |ui| {
                if ui
                    .button(rich_text("Back".to_string(), Module(&MODULE), &font))
                    .clicked()
                {
                    ui_game_menu_writer.send(GameMenuEvent(UiAction::Disable));
                }
                if ui
                    .button(rich_text("Menu".to_string(), Module(&MODULE), &font))
                    .clicked()
                {
                    next_state_lobby.set(LobbyState::None);
                    ui_game_menu_writer.send(GameMenuEvent(UiAction::Disable));
                    next_state_province.set(ProvinceState::Menu);
                    next_state_ui.set(UiState::Menu);
                }
            });
    }
}
