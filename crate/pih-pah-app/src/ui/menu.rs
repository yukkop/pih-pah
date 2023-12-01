use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::lobby::LobbyState;
use crate::province::{self, ProvinceState};
use crate::ui::{GameMenuEvent, rich_text, TRANSPARENT, UiAction};
use crate::util::ResourceAction;
use crate::util::i18n::Uniq::Module;

lazy_static::lazy_static! {
    static ref MODULE: &'static str = module_path!().splitn(3, ':').nth(2).unwrap_or(module_path!());
}

#[derive(Event)]
pub struct MenuEvent(pub ResourceAction);

#[derive(Resource)]
struct State {
    is_active: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_active: false,
        }
    }
}

pub struct MenuPlugins;

impl Plugin for MenuPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MenuEvent>()
            .init_resource::<State>()
            .add_systems(Update, handle_action)
            .add_systems(Update, handle_state);
    }
}

fn handle_action(
    mut reader: EventReader<MenuEvent>,
    mut state: ResMut<State>,
) {
    for MenuEvent(action) in reader.read() {
        match action {
            ResourceAction::Load => {
                state.is_active = true;
            },
            ResourceAction::Unload => {
                state.is_active = false;
            },
        }
    }
}

fn handle_state(
    mut next_state_lobby: ResMut<NextState<LobbyState>>,
    mut next_state_province: ResMut<NextState<ProvinceState>>,
    mut context: EguiContexts,
    mut exit: EventWriter<AppExit>,
    state: Res<State>,
    mut ui_menu_writer: EventWriter<MenuEvent>,
    mut ui_game_menu_writer: EventWriter<GameMenuEvent>,
) {
    let ctx = context.ctx_mut();

    let font = egui::FontId {
        family: egui::FontFamily::Monospace,
        ..default()
    };

    if state.is_active {
        egui::Window::new(rich_text(
            "Menu".to_string(),
            Module(&MODULE),
            &font))
            .frame(*TRANSPARENT)
            .anchor(egui::Align2::LEFT_BOTTOM, [10., -10.])
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .show(ctx, |ui| {
                if ui.button(rich_text(
                    "Shooting range".to_string(),
                    Module(&MODULE),
                    &font)).clicked() {
                    next_state_lobby.set(LobbyState::Single);
                    ui_game_menu_writer.send(GameMenuEvent(UiAction::Load));
                    ui_menu_writer.send(MenuEvent(ResourceAction::Unload));
                    next_state_province.set(ProvinceState::ShootingRange);
                }
                if ui.button(rich_text(
                    "Multiplayer".to_string(),
                    Module(&MODULE),
                    &font)).clicked() {

                }
                if ui.button(rich_text(
                    "Settings".to_string(),
                    Module(&MODULE),
                    &font)).clicked() {

                }
                if ui.button(rich_text(
                    "Exit".to_string(),
                    Module(&MODULE),
                    &font)).clicked() {
                    exit.send(AppExit);
                }
            });
    }
}
