use bevy::prelude::*;
use egui::FontId;

use crate::game::GameState;
use crate::ui::MenuAction;
use crate::ui::MenuActions;
use crate::util::Uniq;
use crate::util::module_cut_out;
use crate::util::trans;
use std::sync::Arc;
use bevy_egui::EguiContexts;
use bevy_egui::EguiPlugin;

use crate::util::Uniq::Module;
use crate::util::module_cat_off;
use crate::ui::egui::TRANSPARENT;

lazy_static::lazy_static! {
    /// Module path for this module, 
    /// Use it for translate text like `trans("text", Module(&MODULE))`
    /// where `Module(&MODULE)` is `Uniq` id for translate text
    static ref MODULE: &'static str = module_cat_off(module_cut_out(module_path!(), "egui"));
}

/// Plugin that registers all egui view layer that wrapp ui logic 
pub struct EguiPlugins;

impl Plugin for EguiPlugins {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.add_systems(Update, menu.run_if(in_state(GameState::Menu)));
    }
}

pub fn rich_text(text: impl Into<Arc<String>>, uniq: Uniq, font: &FontId) -> egui::RichText {
    egui::RichText::new(trans(text.into(), uniq)).font(font.clone())
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
                commands.run_system(menu_actions.get(MenuAction::OpenEditor));
            }
            if ui
                .button(rich_text("Options".to_string(), Module(&MODULE), &font))
                .clicked()
            {
            }
            if ui
                .button(rich_text("Exit".to_string(), Module(&MODULE), &font))
                .clicked()
            {
                commands.run_system(menu_actions.get(MenuAction::Exit));
            }
        });
}