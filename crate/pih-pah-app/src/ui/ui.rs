use crate::ui::menu::MenuPlugins;
use crate::ui::GameMenuPlugins;
use crate::util::i18n::{trans, Uniq};
use bevy::prelude::*;
use bevy_egui::egui::FontId;
use bevy_egui::{egui, EguiContexts};
use std::sync::Arc;

use super::DebugUiPlugins;

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
        app.add_state::<UiState>()
            .add_plugins((DebugUiPlugins, MenuPlugins, GameMenuPlugins));
    }
}


pub fn rich_text(text: impl Into<Arc<String>>, uniq: Uniq, font: &FontId) -> egui::RichText {
    egui::RichText::new(trans(text.into(), uniq)).font(font.clone())
}

fn setup() {}
