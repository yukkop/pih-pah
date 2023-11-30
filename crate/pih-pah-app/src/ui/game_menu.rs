use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{province, ui};
use crate::ui::{rich_text, TRANSPARENT, UiAction};
use crate::util::ResourceAction;
use crate::util::i18n::Uniq::Module;

lazy_static::lazy_static! {
    static ref module: &'static str = module_path!().splitn(3, ':').nth(2).unwrap_or(module_path!());
}

#[derive(Event)]
pub struct GameMenuEvent(pub UiAction);

#[derive(Resource)]
struct State {
    is_active: bool,
    is_loaded: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_active: false,
            is_loaded: false,
        }
    }
}

pub struct GameMenuPlugins;

impl Plugin for GameMenuPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameMenuEvent>()
            .init_resource::<State>()
            .add_systems(Update, handle_action)
            .add_systems(Update, handle_state);
    }
}

fn handle_action(
    mut reader: EventReader<GameMenuEvent>,
    mut state: ResMut<State>,
) {
    for GameMenuEvent(action) in reader.read() {
        match action {
            UiAction::Load => {
                state.is_loaded = true;
            },
            UiAction::Unload => {
                state.is_loaded = false;
            },
            UiAction::Enable => {
                if state.is_loaded {
                    state.is_active = true;
                }
            }
            UiAction::Disable => {
                if state.is_loaded {
                    state.is_active = false;
                }
            }
            UiAction::Toggle => {
                if state.is_loaded {
                    state.is_active = !state.is_active;
                }
            }
        }
    }
}

fn handle_state(
    mut context: EguiContexts,
    state: Res<State>,
    mut ui_menu_writer: EventWriter<ui::MenuEvent>,
    mut province_menu_writer: EventWriter<province::MenuEvent>,
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
            Module(&module),
            &font))
            .frame(*TRANSPARENT)
            .anchor(egui::Align2::LEFT_BOTTOM, [10., -10.])
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .show(ctx, |ui| {
                if ui.button(rich_text(
                    "Back".to_string(),
                    Module(&module),
                    &font)).clicked() {
                    ui_game_menu_writer.send(GameMenuEvent(UiAction::Disable));
                }
                if ui.button(rich_text(
                    "Menu".to_string(),
                    Module(&module),
                    &font)).clicked() {
                    ui_game_menu_writer.send(GameMenuEvent(UiAction::Disable));
                    ui_game_menu_writer.send(GameMenuEvent(UiAction::Unload));
                    ui_menu_writer.send(ui::MenuEvent(ResourceAction::Load));
                    province_menu_writer.send(province::MenuEvent(ResourceAction::Load));
                }
            });
    }
}