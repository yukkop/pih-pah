use crate::ui::menu::MenuPlugins;
use crate::ui::GameMenuPlugins;
use crate::util::i18n::{trans, Uniq};
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_egui::egui::FontId;
use bevy_egui::{egui, EguiContexts};
use std::sync::Arc;

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
            .add_state::<MouseGrabState>()
            .add_plugins((MenuPlugins, GameMenuPlugins))
            .add_systems(OnEnter(MouseGrabState::Enable), grab_mouse_on)
            .add_systems(OnEnter(MouseGrabState::Disable), grab_mouse_off)
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

fn grab_mouse_on(
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

fn grab_mouse_off(
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;
}

fn setup() {}
