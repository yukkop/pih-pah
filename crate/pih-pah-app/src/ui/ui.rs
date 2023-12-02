use std::sync::Arc;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiContexts};
use bevy_egui::egui::FontId;
use crate::ui::GameMenuPlugins;
use crate::ui::menu::MenuPlugins;
use crate::util::i18n::{trans, Uniq};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum UiState {
    #[default]
    Menu,
    GameMenu,
}

pub enum UiAction {
    ///
    Enable = 2,
    ///
    Disable = 3,
    ///
    Toggle = 4,
}

pub struct UiPlugins;

impl Plugin for UiPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_state::<UiState>()
            .add_plugins((MenuPlugins, GameMenuPlugins))
            .add_systems(Startup, (setup, set_egui_debug));
    }
}

fn set_egui_debug(mut context: EguiContexts) {
    context.ctx_mut().set_style(egui::Style {
      debug: egui::style::DebugOptions {
        debug_on_hover: true,
        ..default()
      },
      ..default()
    });
}

pub fn rich_text(text: impl Into<Arc<String>>, uniq: Uniq, font: &FontId) -> egui::RichText {
    egui::RichText::new(trans(text.into(), uniq)).font(font.clone())
}

fn setup() {

}