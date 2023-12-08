use crate::ui::menu::MenuPlugins;
use crate::ui::GameMenuPlugins;
use crate::util::i18n::{trans, Uniq};
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_egui::egui::FontId;
use std::sync::Arc;

use super::DebugUiPlugins;

#[derive(Debug, Clone, Copy, Resource, PartialEq, Deref, DerefMut)]
pub struct ViewportRect(egui::Rect);

impl Default for ViewportRect {
    fn default() -> Self {
        Self(egui::Rect::from_min_size(
            Default::default(),
            Default::default(),
        ))
    }
}

impl ViewportRect {
    pub fn set(&mut self, rect: egui::Rect) {
        self.0 = rect;
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MouseGrabState {
    Enable,
    #[default]
    Disable,
}

impl MouseGrabState {
    pub fn toggle(&mut self) -> Self {
        match self {
            MouseGrabState::Enable => *self = MouseGrabState::Disable,
            MouseGrabState::Disable => *self = MouseGrabState::Enable,
        }
        *self
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum UiState {
    #[default]
    Menu,
    GameMenu,
}

pub struct UiPlugins;

impl Plugin for UiPlugins {
    fn build(&self, app: &mut App) {
        app.add_state::<UiState>()
            .add_state::<MouseGrabState>()
            .init_resource::<ViewportRect>()
            .add_plugins((DebugUiPlugins, MenuPlugins, GameMenuPlugins))
            .add_systems(OnEnter(MouseGrabState::Enable), grab_mouse_on)
            .add_systems(OnEnter(MouseGrabState::Disable), grab_mouse_off);
    }
}

pub fn frame_rect(mut windows: Query<&Window>, mut ui_frame_rect: ResMut<ViewportRect>) {
    let window = windows.single_mut();
    let window_size = egui::vec2(window.width(), window.height());

    ui_frame_rect.set(egui::Rect::from_min_size(Default::default(), window_size));
}

pub fn rich_text(text: impl Into<Arc<String>>, uniq: Uniq, font: &FontId) -> egui::RichText {
    egui::RichText::new(trans(text.into(), uniq)).font(font.clone())
}

fn grab_mouse_on(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

fn grab_mouse_off(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;
}
