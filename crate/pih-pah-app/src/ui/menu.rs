use crate::lobby::{ClientResource, HostResource, LobbyState};
use crate::province::ProvinceState;
use crate::settings::{ApplySettings, ExemptSettings, Settings};
use crate::ui::{rich_text, TRANSPARENT};
use crate::util::i18n::Uniq::Module;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::Window;
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContexts};

use super::UiState;

lazy_static::lazy_static! {
    static ref MODULE: &'static str = module_path!().splitn(3, ':').nth(2).unwrap_or(module_path!());
}

enum MultiplayerState {
    Create = 0,
    Join = 1,
}

#[derive(Resource)]
struct State {
    multiplayer_state: MultiplayerState,
    host_port: String,
    join_address: String,
    username: String,
}

#[derive(Default, Debug, Hash, States, PartialEq, Eq, Clone, Copy)]
enum WindowState {
    #[default]
    None,
    Multiplayer,
    Settings,
}

impl Default for State {
    fn default() -> Self {
        Self {
            multiplayer_state: MultiplayerState::Create,
            host_port: "5000".to_string(),
            join_address: "127.0.0.1:5000".to_string(),
            username: "noname".to_string(),
        }
    }
}

pub struct MenuPlugins;

impl Plugin for MenuPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<State>()
            .add_state::<WindowState>()
            .add_systems(Update, menu.run_if(in_state(UiState::Menu)))
            .add_systems(
                Update,
                settings_window
                    .run_if(in_state(UiState::Menu).and_then(in_state(WindowState::Settings))),
            )
            .add_systems(OnExit(WindowState::Settings), exempt_setting)
            .add_systems(
                Update,
                multiplayer_window
                    .run_if(in_state(UiState::Menu).and_then(in_state(WindowState::Multiplayer))),
            );
    }
}

fn menu(
    mut next_state_lobby: ResMut<NextState<LobbyState>>,
    mut next_state_ui: ResMut<NextState<UiState>>,
    mut next_state_province: ResMut<NextState<ProvinceState>>,
    mut next_state_menu_window: ResMut<NextState<WindowState>>,
    mut context: EguiContexts,
    mut exit: EventWriter<AppExit>,
) {
    let ctx = context.ctx_mut();

    let font = egui::FontId {
        family: egui::FontFamily::Monospace,
        ..default()
    };

    egui::Window::new(rich_text("Menu".to_string(), Module(&MODULE), &font))
        .frame(*TRANSPARENT)
        .anchor(egui::Align2::LEFT_BOTTOM, [10., -10.])
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(ctx, |ui| {
            if ui
                .button(rich_text(
                    "Shooting range".to_string(),
                    Module(&MODULE),
                    &font,
                ))
                .clicked()
            {
                next_state_ui.set(UiState::GameMenu);
                next_state_province.set(ProvinceState::ShootingRange);
                next_state_lobby.set(LobbyState::Single);
            }
            if ui
                .button(rich_text("Multiplayer".to_string(), Module(&MODULE), &font))
                .clicked()
            {
                next_state_menu_window.set(WindowState::Multiplayer);
            }
            if ui
                .button(rich_text("Settings".to_string(), Module(&MODULE), &font))
                .clicked()
            {
                next_state_menu_window.set(WindowState::Settings);
            }
            if ui
                .button(rich_text("Exit".to_string(), Module(&MODULE), &font))
                .clicked()
            {
                exit.send(AppExit);
            }
        });
}

#[allow(clippy::too_many_arguments)]
fn multiplayer_window(
    mut next_state_lobby: ResMut<NextState<LobbyState>>,
    mut next_state_ui: ResMut<NextState<UiState>>,
    mut next_state_province: ResMut<NextState<ProvinceState>>,
    mut next_state_menu_window: ResMut<NextState<WindowState>>,
    mut context: EguiContexts,
    mut state: ResMut<State>,
    mut windows: Query<&Window>,
    mut host_resource: ResMut<HostResource>,
    mut client_resource: ResMut<ClientResource>,
) {
    let window = windows.single_mut();
    let window_size = egui::vec2(window.width(), window.height());

    let ctx = context.ctx_mut();

    let font = egui::FontId {
        family: egui::FontFamily::Monospace,
        ..default()
    };

    let egui_window_size = egui::vec2(400.0, 200.0); // Set your desired egui window size

    let center_position = egui::pos2(window_size.x / 2.0, window_size.y / 2.0);

    egui::Window::new(rich_text("Multiplayer".to_string(), Module(&MODULE), &font))
        .pivot(Align2::CENTER_CENTER)
        .fixed_size(egui_window_size)
        .fixed_pos(center_position)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(ctx, |ui| {
            match state.multiplayer_state {
                MultiplayerState::Create => {
                    ui.horizontal(|ui| {
                        ui.label(rich_text("Create".to_string(), Module(&MODULE), &font));
                        if ui
                            .button(rich_text("Join".to_string(), Module(&MODULE), &font))
                            .clicked()
                        {
                            state.multiplayer_state = MultiplayerState::Join;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Port:");
                        ui.text_edit_singleline(&mut state.host_port);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Username:");
                        ui.text_edit_singleline(&mut state.username);
                    });
                    if ui
                        .button(rich_text("Create".to_string(), Module(&MODULE), &font))
                        .clicked()
                    {
                        host_resource.address =
                            Some(format!("127.0.0.1:{}", state.host_port.clone()));
                        host_resource.username = Some(state.username.clone());
                        next_state_menu_window.set(WindowState::None);
                        next_state_lobby.set(LobbyState::Host);
                        next_state_province.set(ProvinceState::ShootingRange);
                        next_state_ui.set(UiState::GameMenu);
                    }
                }
                MultiplayerState::Join => {
                    ui.horizontal(|ui| {
                        if ui
                            .button(rich_text("Create".to_string(), Module(&MODULE), &font))
                            .clicked()
                        {
                            state.multiplayer_state = MultiplayerState::Create;
                        }
                        ui.label(rich_text("Join".to_string(), Module(&MODULE), &font));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Address:");
                        ui.text_edit_singleline(&mut state.join_address);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Username:");
                        ui.text_edit_singleline(&mut state.username);
                    });
                    if ui
                        .button(rich_text("Connect".to_string(), Module(&MODULE), &font))
                        .clicked()
                    {
                        client_resource.address = Some(state.join_address.clone());
                        client_resource.username = Some(state.username.clone());
                        next_state_menu_window.set(WindowState::None);
                        state.multiplayer_state = MultiplayerState::Create;
                        next_state_lobby.set(LobbyState::Client);
                        next_state_province.set(ProvinceState::ShootingRange);
                        next_state_ui.set(UiState::GameMenu);
                    }
                }
            }
            if ui
                .button(rich_text("Back".to_string(), Module(&MODULE), &font))
                .clicked()
            {
                next_state_menu_window.set(WindowState::None);
            }
        });
}

fn settings_window(
    mut next_state_menu_window: ResMut<NextState<WindowState>>,
    mut context: EguiContexts,
    mut windows: Query<&Window>,
    mut settings: ResMut<Settings>,
    mut settings_applying: EventWriter<ApplySettings>,
) {
    let window = windows.single_mut();
    let window_size = egui::vec2(window.width(), window.height());

    let ctx = context.ctx_mut();

    let font = egui::FontId {
        family: egui::FontFamily::Monospace,
        ..default()
    };

    let egui_window_size = egui::vec2(400.0, 200.0); // Set your desired egui window size

    let center_position = egui::pos2(window_size.x / 2.0, window_size.y / 2.0);

    egui::Window::new(rich_text("Settings".to_string(), Module(&MODULE), &font))
        .pivot(Align2::CENTER_CENTER)
        .fixed_size(egui_window_size)
        .fixed_pos(center_position)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Music: {}", settings.music_volume));
                ui.add(egui::Slider::new(&mut settings.music_volume, 0.0..=200.0).text("%"));
            });
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
                    settings_applying.send(ApplySettings);
                }
                if ui
                    .button(rich_text("Ok".to_string(), Module(&MODULE), &font))
                    .clicked()
                {
                    settings_applying.send(ApplySettings);
                    next_state_menu_window.set(WindowState::None);
                }
            });
        });
}

fn exempt_setting(mut event: EventWriter<ExemptSettings>) {
    event.send(ExemptSettings);
}