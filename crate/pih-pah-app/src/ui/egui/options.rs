use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::Align2;
use crate::game::GameState;
use crate::ui::{MenuWindow, OptionsActions, OptionsAction};
use crate::{rich_text, define_module};
use crate::util::Uniq::Module;

use crate::option::Options;

use super::rich_text;

define_module!();

/// Plugin that registers all egui view layer that wrapp ui logic 
pub struct OptionsPlugins;

impl Plugin for OptionsPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, settings_window.run_if(in_state(MenuWindow::Options)).run_if(in_state(GameState::Menu)));
    }
}

fn settings_window(
    mut commands: Commands,
    mut context: EguiContexts,
    mut windows: Query<&Window>,
    mut options: ResMut<Options>,
    options_action: Res<OptionsActions>,
) {
    let window = windows.single_mut();
    let window_size = egui::vec2(window.width(), window.height());

    let ui_frame_rect = egui::Rect::from_min_size(Default::default(), window_size);

    let frame_size = ui_frame_rect.max - ui_frame_rect.min;

    let ctx = context.ctx_mut();

    let font = egui::FontId {
        family: egui::FontFamily::Monospace,
        ..default()
    };

    let egui_window_size = egui::vec2(400.0, 200.0); // Set your desired egui window size

    let center_position = egui::pos2(frame_size.x / 2.0, frame_size.y / 2.0);

    egui::Window::new(rich_text!("Settings", &MODULE, &font))
        .pivot(Align2::CENTER_CENTER)
        .fixed_size(egui_window_size)
        .fixed_pos(center_position)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Music: {}", options.music_volume));
                ui.add(egui::Slider::new(&mut options.music_volume, 0.0..=200.0).text("%"));
            });
            ui.horizontal(|ui| {
                if ui
                    .button(rich_text!("Cancel", &MODULE, &font))
                    .clicked()
                {
                    commands.run_system(options_action.get(OptionsAction::Exempt));
                    commands.run_system(options_action.get(OptionsAction::Close));
                }
                if ui
                    .button(rich_text!("Apply", &MODULE, &font))
                    .clicked()
                {
                    commands.run_system(options_action.get(OptionsAction::Apply));
                }
                if ui
                    .button(rich_text!("Ok", &MODULE, &font))
                    .clicked()
                {
                    commands.run_system(options_action.get(OptionsAction::Ok));
                }
            });
        });
}