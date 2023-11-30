use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::ui::{rich_text, TRANSPARENT};
use crate::util::i18n::Uniq;
use crate::util::ResourceAction;
use crate::util::i18n::Uniq::Module;

lazy_static::lazy_static! {
    static ref module: &'static str = module_path!().splitn(3, ':').nth(2).unwrap_or(module_path!());
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
    mut context: EguiContexts,
    state: Res<State>,
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
                    "Shutting range".to_string(),
                    Module(&module),
                    &font)).clicked() {

                }
                if ui.button(rich_text(
                    "Multiplayer".to_string(),
                    Module(&module),
                    &font)).clicked() {

                }
                if ui.button(rich_text(
                    "Settings".to_string(),
                    Module(&module),
                    &font)).clicked() {

                }
                if ui.button(rich_text(
                    "Exit".to_string(),
                    Module(&module),
                    &font)).clicked() {

                }
            });
    }
}
