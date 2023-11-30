use std::sync::Arc;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::FontId;
use crate::ui::GameMenuPlugins;
use crate::ui::menu::MenuPlugins;
use crate::util::i18n::{trans, Uniq};

pub enum UiAction {
    ///
    Load,
    ///
    Unload,
    ///
    Enable,
    ///
    Disable,
    ///
    Toggle,
}

pub struct UiPlugins;

impl Plugin for UiPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((MenuPlugins, GameMenuPlugins))
            .add_systems(Startup, setup);
    }
}

pub fn rich_text(text: impl Into<Arc<String>>, uniq: Uniq, font: &FontId) -> egui::RichText {
    egui::RichText::new(trans(text.into(), uniq)).font(font.clone())
}

fn setup() {
}
