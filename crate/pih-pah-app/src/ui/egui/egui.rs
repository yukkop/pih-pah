use bevy::prelude::*;
use egui::FontId;

use crate::util::trans;
use crate::util::Uniq;
use bevy_egui::EguiPlugin;
use std::sync::Arc;

use super::menu::MenuPlugins;
use super::options::OptionsPlugins;

/// Plugin that registers all egui view layer that wrapp ui logic
pub struct EguiPlugins;

impl Plugin for EguiPlugins {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.add_plugins((MenuPlugins, OptionsPlugins));
    }
}

pub fn rich_text(text: impl Into<Arc<String>>, uniq: Uniq, font: &FontId) -> egui::RichText {
    egui::RichText::new(trans(text.into(), uniq)).font(font.clone())
}
