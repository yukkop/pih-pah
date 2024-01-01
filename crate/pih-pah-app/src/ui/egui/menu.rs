use bevy::prelude::*;
use bevy_egui::EguiContexts;
use crate::define_module;
use crate::game::GameState;
use crate::ui::MenuAction;
use crate::ui::MenuActions;

use crate::ui::egui::TRANSPARENT;
use crate::util::Uniq::Module;

use super::rich_text;

define_module!();

/// Plugin that registers all egui view layer that wrapp ui logic 
pub struct MenuPlugins;

impl Plugin for MenuPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, menu.run_if(in_state(GameState::Menu)));
    }
}

/// System for update menu 
/// 
/// It must only draw ui and runs metods from `MenuActions` to change game states
fn menu(
    mut commands: Commands,
    mut context: EguiContexts,
    mut windows: Query<&Window>,
    menu_actions: Res<MenuActions>,
) {
    let ctx = context.ctx_mut();

    let font = egui::FontId {
        family: egui::FontFamily::Monospace,
        ..default()
    };

    let window = windows.single_mut();
    let window_size = egui::vec2(window.width(), window.height());

    let ui_frame_rect = egui::Rect::from_min_size(Default::default(), window_size);

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
                .button(rich_text(
                    "Editor".to_string(),
                    Module(&MODULE),
                    &font,
                ))
                .clicked()
            {
                commands.run_system(menu_actions.get(MenuAction::StartLevelEditing));
            }
            if ui
                .button(rich_text("Options".to_string(), Module(&MODULE), &font))
                .clicked()
            {
                commands.run_system(menu_actions.get(MenuAction::OpenOptions));
            }
            if ui
                .button(rich_text("Exit".to_string(), Module(&MODULE), &font))
                .clicked()
            {
                commands.run_system(menu_actions.get(MenuAction::Exit));
            }
        });
}
