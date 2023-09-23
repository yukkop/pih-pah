use crate::feature::ui::HudPlugins;
use bevy::prelude::*;
use bevy_egui::{
  egui,
  EguiContexts,
};

use bevy::prelude::*;
pub struct UiPlugins;

/// EguiPlugin nessesarly
impl Plugin for UiPlugins {
  fn build(&self, app: &mut App) {
    app.add_plugins(HudPlugins);
  }
}

pub struct UiDebugPlugins;

/// EguiPlugin nessesarly
impl Plugin for UiDebugPlugins {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<UiDebugState>()
      .add_systems(Update, debug_preferences_ui)
      .add_systems(Update, disable_egui_debug.before(enable_egui_debug))
      .add_systems(Update, enable_egui_debug.run_if(is_egui_ui_debug_enabled))
      .add_systems(Update, fps_ui.run_if(is_fps_ui_enabled))
    ;
  }
}

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

#[derive(Resource)]
pub struct UiDebugState {
  pub is_fps_ui_enabled: bool,
  pub is_preferences_window_open: bool,
  pub is_egui_ui_debug_enabled: bool,
}

impl Default for UiDebugState {
  fn default() -> Self {
    Self {
      is_fps_ui_enabled: true,
      is_preferences_window_open: true,
      is_egui_ui_debug_enabled: true,
    }
  }
}

fn debug_preferences_ui(
  mut contexts: EguiContexts,
  diagnostics: Res<DiagnosticsStore>,
  mut ui_state: ResMut<UiDebugState>,
) {
  let ctx = contexts.ctx_mut();

  egui::Window::new("Preferences")
    .frame(*crate::lib::egui_frame_preset::NO_SHADOW_FRAME)
    .anchor(egui::Align2::RIGHT_TOP, egui::vec2(0., 0.))
    .vscroll(true)
    .show(ctx, |ui| {
      ui.checkbox(&mut ui_state.is_fps_ui_enabled, "show fps");
      ui.checkbox(&mut ui_state.is_egui_ui_debug_enabled, "debug egui borders (hold alt)");
    });
}

fn set_egui_debug(context: &mut egui::Context, debug_on_hover: bool) {
  context.set_style(egui::Style {
    debug: egui::style::DebugOptions {
      debug_on_hover,
      ..default()
    },
    ..default()
  });
}

fn is_egui_ui_debug_enabled(ui_debug_state: Res<UiDebugState>) -> bool {
  ui_debug_state.is_egui_ui_debug_enabled
}
fn disable_egui_debug(mut contexts: EguiContexts, key_input: Res<Input<KeyCode>>, ui_debug_state: Res<UiDebugState>) {
  if key_input.just_released(KeyCode::AltLeft) || key_input.just_released(KeyCode::AltRight) || (ui_debug_state.is_changed() && !ui_debug_state.is_egui_ui_debug_enabled) {
    set_egui_debug(contexts.ctx_mut(), false);
  }
}
fn enable_egui_debug(mut contexts: EguiContexts, key_input: Res<Input<KeyCode>>) {
  if key_input.just_pressed(KeyCode::AltLeft) || key_input.just_pressed(KeyCode::AltRight) {
    set_egui_debug(contexts.ctx_mut(), true);
  }
}

fn is_fps_ui_enabled(ui_debug_state: Res<UiDebugState>) -> bool {
  ui_debug_state.is_fps_ui_enabled 
}
fn fps_ui(mut contexts: EguiContexts, diagnostics: Res<DiagnosticsStore>) {
  let fps_diagnostic = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS);
  let frame_time_diagnostic = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME);
  let fps = fps_diagnostic
    .and_then(|fps| fps.value())
    .map(|fps| fps.to_string())
    .unwrap_or(String::from("N/A"));
  let fps_average = fps_diagnostic
    .and_then(|fps| fps.average())
    .map(|fps| fps.to_string())
    .unwrap_or(String::from("N/A"));
  let fps_smooth = fps_diagnostic
    .and_then(|fps| fps.smoothed())
    .map(|fps| fps.to_string())
    .unwrap_or(String::from("N/A"));
  let frame_time = frame_time_diagnostic 
    .and_then(|ft| ft.value())
    .map(|ft| ft.to_string())
    .unwrap_or(String::from("N/A"));

  let ctx = contexts.ctx_mut();

  egui::CentralPanel::default()
    .frame(*crate::lib::egui_frame_preset::TRANSPARENT)
    .show(ctx, |ui| {
      let font = egui::FontId {
        family: egui::FontFamily::Monospace,
        ..default()
      };
      ui.label(egui::RichText::new(format!("fps         : {fps:.2}")).font(font.clone()));
      ui.label(egui::RichText::new(format!("fps_average : {fps_average:.2}")).font(font.clone()));
      ui.label(egui::RichText::new(format!("fps_smooth  : {fps_smooth:.2}")).font(font.clone()));
      ui.label(egui::RichText::new(format!("frame_time  : {frame_time:.2}")).font(font.clone()));
    });
}
