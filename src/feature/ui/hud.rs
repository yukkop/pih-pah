use bevy::prelude::*;
use bevy_egui::{egui::{self, Color32}, EguiContexts, EguiPlugin, EguiSettings };

pub struct HudPlugins;

impl Plugin for HudPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, ui);
    }
}

fn ui(mut contexts: EguiContexts) {
  let ctx = contexts.ctx_mut();

    let my_frame = egui::containers::Frame {
              fill: Color32::TRANSPARENT,
              ..default()
          };

  egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {
      ui.heading("My Butiful Game 0.1.0");
  });
}
