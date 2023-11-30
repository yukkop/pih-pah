use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::FontId;
use crate::ui::menu::MenuPlugins;

pub struct UiPlugins;

impl Plugin for UiPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MenuPlugins)
            .add_systems(Startup, setup);
    }
}

pub fn rich_text(text: impl Into<String>, font: &FontId) -> egui::RichText {
    egui::RichText::new(text).font(font.clone())
}

fn setup() {
}
