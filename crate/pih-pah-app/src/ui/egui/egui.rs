use bevy::prelude::*;
use egui::FontId;

use crate::game::GameState;
use crate::util::Uniq;
use crate::util::trans;
use bevy::app::AppExit;
use std::sync::Arc;
use bevy_egui::EguiPlugin;
use bevy_egui::EguiContexts;

use crate::util::Uniq::Module;
use crate::util::module_cat;
use crate::ui::egui::TRANSPARENT;

lazy_static::lazy_static! {
    static ref MODULE: &'static str = module_cat(module_path!());
}

pub struct EguiPlugins;

impl Plugin for EguiPlugins {
    fn build(&self, app: &mut App) {
        #[cfg(not(feature = "dev"))]
        app.add_plugins(EguiPlugin);

        app.add_systems(Update, menu.run_if(in_state(GameState::Menu)));
    }
}

pub fn rich_text(text: impl Into<Arc<String>>, uniq: Uniq, font: &FontId) -> egui::RichText {
    egui::RichText::new(trans(text.into(), uniq)).font(font.clone())
}

fn menu(
    mut context: EguiContexts,
    mut windows: Query<&Window>,
    mut exit: EventWriter<AppExit>,
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
                    "Shooting range".to_string(),
                    Module(&MODULE),
                    &font,
                ))
                .clicked()
            {
            }
            if ui
                .button(rich_text("Multiplayer".to_string(), Module(&MODULE), &font))
                .clicked()
            {
            }
            if ui
                .button(rich_text("Settings".to_string(), Module(&MODULE), &font))
                .clicked()
            {
            }
            if ui
                .button(rich_text("Exit".to_string(), Module(&MODULE), &font))
                .clicked()
            {
                exit.send(AppExit);
            }
        });
}

