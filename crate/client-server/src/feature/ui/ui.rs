use crate::feature::ui::HudPlugins;
use bevy_egui::{
  egui::{self, Color32},
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

pub struct FpsPlugins;

/// EguiPlugin nessesarly
impl Plugin for FpsPlugins {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, ui);
  }
}

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

fn ui(mut contexts: EguiContexts, diagnostics: Res<DiagnosticsStore>) {
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

  let ctx = contexts.ctx_mut();

  let my_frame = egui::containers::Frame {
    fill: Color32::TRANSPARENT,
    ..default()
  };

  egui::CentralPanel::default()
    .frame(my_frame)
    .show(ctx, |ui| {
      ui.label(raw);
      ui.label(sma);
      ui.label(ema);
    });
}
