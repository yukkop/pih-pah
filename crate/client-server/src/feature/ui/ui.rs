use crate::feature::ui::HudPlugins;
use bevy_egui::{
  egui,
  EguiContexts,
};

use bevy::prelude::*;
use epaint::Shadow;    
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
      .add_systems(Update, ui_debug_update);
  }
}

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

#[derive(Resource)]
pub struct UiDebugState {
  pub is_fps_window_open: bool,
  pub is_preferences_window_open: bool,
}

impl Default for UiDebugState {
  fn default() -> Self {
    Self {
      is_fps_window_open: true,
      is_preferences_window_open: true,
    }
  }
}

fn ui_debug_update(
  mut contexts: EguiContexts,
  diagnostics: Res<DiagnosticsStore>,
  mut ui_state: ResMut<UiDebugState>
) {
  let ctx = contexts.ctx_mut();

  // preferences 
  let no_shadow_frame = egui::containers::Frame {
    shadow: Shadow::NONE,
    ..default()
  };

  egui::Window::new("Preferences")
    .frame(no_shadow_frame)
    .vscroll(true)
    .show(ctx, |ui| {
      ui.checkbox(&mut ui_state.is_fps_window_open, "FPS");
  });

  // fps
  let (mut raw, mut sma, mut ema): (String, String, String) =
    ("raw: ".into(), "sma: ".into(), "ema:".into());
  if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
    if let Some(raw_value) = fps.value() {
      raw = format!("raw: {raw_value:.2}");
    }
    if let Some(sma_value) = fps.average() {
      sma = format!("sma: {sma_value:.2}");
    }
    if let Some(ema_value) = fps.smoothed() {
      ema = format!("ema: {ema_value:.2}");
    }
  };

  let my_frame = egui::containers::Frame {
    ..default()
  };

  egui::Window::new("FPS")
    .vscroll(true)
    .open(&mut ui_state.is_fps_window_open)
    .frame(my_frame)
    .show(ctx, |ui| {
      ui.label(raw);
      ui.label(sma);
      ui.label(ema);
  });
}
