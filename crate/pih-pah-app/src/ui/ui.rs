use crate::ui::menu::MenuPlugins;
use crate::ui::GameMenuPlugins;
use crate::util::i18n::{trans, Uniq};
use bevy::prelude::*;
use bevy_egui::egui::FontId;
use egui::CentralPanel;
use std::sync::Arc;

use super::DebugUiPlugins;

#[derive(Debug, Clone, Copy, Resource, PartialEq, Deref, DerefMut)]
pub struct UiFrameRect(egui::Rect);

impl Default for UiFrameRect {
    fn default() -> Self {
        Self(egui::Rect::from_min_size(Default::default(), Default::default()))
    }
}

impl UiFrameRect {
    pub fn set(&mut self, rect: egui::Rect) {
        self.0 = rect;
    }
}

#[derive(Component)]
pub struct MainCamera;

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

#[derive(Default, Resource)]
pub struct UiBase(pub CentralPanel); 

pub struct UiPlugins;

impl Plugin for UiPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_state::<UiState>()
            .init_resource::<UiFrameRect>()
            .insert_resource(UiBase(egui::CentralPanel::default()))
            .add_plugins((DebugUiPlugins, MenuPlugins, GameMenuPlugins))
            .add_systems(Startup, frame_rect);
    }
}

pub fn frame_rect(
    mut windows: Query<&Window>,
    mut ui_frame_rect: ResMut<UiFrameRect>,
) {
    let window = windows.single_mut();
    let window_size = egui::vec2(window.width(), window.height());

    ui_frame_rect.set(egui::Rect::from_min_size(Default::default(), window_size));
}

pub fn rich_text(text: impl Into<Arc<String>>, uniq: Uniq, font: &FontId) -> egui::RichText {
    egui::RichText::new(trans(text.into(), uniq)).font(font.clone())
}