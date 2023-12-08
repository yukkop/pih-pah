use crate::lobby::host::ChangeProvinceServerEvent;
use crate::lobby::LobbyState;
use crate::province::ProvinceState;
use crate::settings::{ApplySettings, ExemptSettings, Settings};
use crate::ui::{rich_text, TRANSPARENT};
use crate::util::i18n::Uniq::Module;
use bevy::prelude::*;
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContexts};

use super::{UiState, ViewportRect};

lazy_static::lazy_static! {
    static ref MODULE: &'static str = module_path!().splitn(3, ':').nth(2).unwrap_or(module_path!());
}

#[derive(Resource)]
struct EguiState {
    is_active: bool,
    selected_map: ProvinceState,
    selected_map_applied: ProvinceState,
}

impl Default for EguiState {
    fn default() -> Self {
        Self {
            is_active: false,
            selected_map: ProvinceState::ShootingRange,
            selected_map_applied: ProvinceState::ShootingRange,
        }
    }
}

#[derive(Default, Debug, Hash, States, PartialEq, Eq, Clone, Copy)]
pub enum GameMenuActionState {
    Enable,
    #[default]
    Disable,
}

impl GameMenuActionState {
    pub fn toggle(&mut self) -> Self {
        match self {
            GameMenuActionState::Enable => *self = GameMenuActionState::Disable,
            GameMenuActionState::Disable => *self = GameMenuActionState::Enable,
        }
        *self
    }
}

#[derive(Default, Debug, Hash, States, PartialEq, Eq, Clone, Copy)]
enum WindowState {
    #[default]
    None,
    Settings,
}

pub struct GameMenuPlugins;

impl Plugin for GameMenuPlugins {
    fn build(&self, app: &mut App) {
        app.add_state::<WindowState>()
            .add_state::<GameMenuActionState>()
            .init_resource::<EguiState>()
            .add_systems(
                Update,
                menu.run_if(
                    in_state(UiState::GameMenu).and_then(in_state(GameMenuActionState::Enable)),
                ),
            )
            .add_systems(
                Update,
                settings_window
                    .run_if(in_state(UiState::GameMenu).and_then(in_state(WindowState::Settings))),
            )
            .add_systems(OnExit(WindowState::Settings), exempt_setting);
    }
}

#[allow(clippy::too_many_arguments)]
fn menu(
    mut next_state_lobby: ResMut<NextState<LobbyState>>,
    mut next_state_ui: ResMut<NextState<UiState>>,
    mut next_state_game_menu_action: ResMut<NextState<GameMenuActionState>>,
    mut next_state_menu_window: ResMut<NextState<WindowState>>,
    mut next_state_province: ResMut<NextState<ProvinceState>>,
    mut context: EguiContexts,
    mut state: ResMut<EguiState>,
    ui_frame_rect: ResMut<ViewportRect>,
    mut windows: Query<&Window>,
) {
    let ctx = context.ctx_mut();

    let font = egui::FontId {
        family: egui::FontFamily::Monospace,
        ..default()
    };

    let window = windows.single_mut();
    let window_size = egui::vec2(window.width(), window.height());

    egui::Window::new(rich_text("Menu".to_string(), Module(&MODULE), &font))
        .frame(*TRANSPARENT)
        .anchor(
            egui::Align2::LEFT_BOTTOM,
            [
                ui_frame_rect.min.x + 10.,
                (window_size.y - ui_frame_rect.max.y) * -1. - 10.,
            ],
        )
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(ctx, |ui| {
            if ui
                .button(rich_text("Back".to_string(), Module(&MODULE), &font))
                .clicked()
            {
                next_state_menu_window.set(WindowState::None);
                next_state_game_menu_action.set(GameMenuActionState::Disable);
            }
            if ui
                .button(rich_text("Settings".to_string(), Module(&MODULE), &font))
                .clicked()
            {
                next_state_menu_window.set(WindowState::Settings);
            }
            if ui
                .button(rich_text("Menu".to_string(), Module(&MODULE), &font))
                .clicked()
            {
                state.is_active = false;
                next_state_game_menu_action.set(GameMenuActionState::Disable);
                next_state_menu_window.set(WindowState::None);
                next_state_lobby.set(LobbyState::None);
                next_state_province.set(ProvinceState::Menu);
                next_state_ui.set(UiState::Menu);
            }
        });
}

#[allow(clippy::too_many_arguments)]
fn settings_window(
    mut next_state_menu_window: ResMut<NextState<WindowState>>,
    mut next_state_province: ResMut<NextState<ProvinceState>>,
    mut context: EguiContexts,
    // mut windows: Query<&Window>,
    mut settings: ResMut<Settings>,
    mut state: ResMut<EguiState>,
    lobby_state: Res<State<LobbyState>>,
    ui_frame_rect: ResMut<ViewportRect>,
    mut settings_applying: EventWriter<ApplySettings>,
    mut change_province: EventWriter<ChangeProvinceServerEvent>,
) {
    // let window = windows.single_mut();
    // let window_size = egui::vec2(window.width(), window.height());
    let frame_size = ui_frame_rect.max - ui_frame_rect.min;

    let ctx = context.ctx_mut();

    let font = egui::FontId {
        family: egui::FontFamily::Monospace,
        ..default()
    };

    let egui_window_size = egui::vec2(400.0, 200.0); // Set your desired egui window size

    let center_position = egui::pos2(frame_size.x / 2.0, frame_size.y / 2.0);

    egui::Window::new(rich_text("Settings".to_string(), Module(&MODULE), &font))
        .pivot(Align2::CENTER_CENTER)
        .fixed_size(egui_window_size)
        .fixed_pos(center_position)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(ctx, |ui| {
            ui.label(rich_text("Audio: ".to_string(), Module(&MODULE), &font));
            ui.horizontal(|ui| {
                ui.label(rich_text(
                    format!("Music: {}", settings.music_volume),
                    Module(&MODULE),
                    &font,
                ));
                ui.add(egui::Slider::new(&mut settings.music_volume, 0.0..=200.0).text("%"));
            });
            if *lobby_state.get() != LobbyState::Client {
                ui.label(rich_text("Province: ".to_string(), Module(&MODULE), &font));
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label(rich_text(
                        "Province".to_string(),
                        Module(&MODULE),
                        &font,
                    ))
                    .selected_text(format!("{}", state.selected_map))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut state.selected_map,
                            ProvinceState::ShootingRange,
                            ProvinceState::ShootingRange.to_string(),
                        );
                        ui.selectable_value(
                            &mut state.selected_map,
                            ProvinceState::GravityHell,
                            ProvinceState::GravityHell.to_string(),
                        );
                    });
                });
            }
            ui.horizontal(|ui| {
                if ui
                    .button(rich_text("Cansel".to_string(), Module(&MODULE), &font))
                    .clicked()
                {
                    next_state_menu_window.set(WindowState::None);
                }
                if ui
                    .button(rich_text("Apply".to_string(), Module(&MODULE), &font))
                    .clicked()
                {
                    if state.selected_map_applied != state.selected_map {
                        state.selected_map_applied = state.selected_map;
                        next_state_province.set(state.selected_map);
                        change_province.send(ChangeProvinceServerEvent(state.selected_map));
                    }
                    settings_applying.send(ApplySettings);
                }
                if ui
                    .button(rich_text("Ok".to_string(), Module(&MODULE), &font))
                    .clicked()
                {
                    if state.selected_map_applied != state.selected_map {
                        state.selected_map_applied = state.selected_map;
                        next_state_province.set(state.selected_map);
                        change_province.send(ChangeProvinceServerEvent(state.selected_map));
                    }
                    settings_applying.send(ApplySettings);
                    next_state_menu_window.set(WindowState::None);
                }
            });
        });
}

fn exempt_setting(mut event: EventWriter<ExemptSettings>, mut state: ResMut<EguiState>) {
    state.selected_map = state.selected_map_applied;
    event.send(ExemptSettings);
}
