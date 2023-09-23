use bevy::prelude::*;
use bevy_egui::{
  egui::{self, Color32, Stroke, Margin, Rounding},
  EguiContexts,
};
use epaint::Shadow;

use crate::feature::multiplayer::Lobby;

pub struct HudPlugins;

impl Plugin for HudPlugins {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, ui);
  }
}

fn ui(
  mut contexts: EguiContexts,
  lobby: Res<Lobby>,
) {
  let ctx = contexts.ctx_mut();

  let transparent_frame = egui::containers::Frame {
    fill: Color32::TRANSPARENT,
    stroke: Stroke::NONE,
    shadow: Shadow::NONE,
    outer_margin: Margin { left: 0., right: 0., top: 0., bottom: 0. }, 
    inner_margin: Margin { left: 0., right: 0., top: 0., bottom: 0. }, 
    rounding: Rounding { nw: 0., ne: 0., sw: 0., se: 0. },
  };

  egui::TopBottomPanel::top("Top")
    .show_separator_line(false)
    .frame(transparent_frame)
    .show(ctx, |ui| 
  {
    ui.heading("Pih-Pah 0.1.0")
  });

  egui::SidePanel::right("Users")
    .show_separator_line(false)
    .frame(transparent_frame)
    .show(ctx, |ui| {
      for (player_id, player_data) in lobby.players.iter() {
        // TODO Color32
        ui.colored_label(
          Color32::from_rgb(
            (player_data.color.r() * 255.) as u8,
            (player_data.color.g() * 255.) as u8,
            (player_data.color.b() * 255.) as u8,
          ),
          format!("{player_id}")
        );
      }
    });

  egui::CentralPanel::default()
    .frame(transparent_frame)
    .show(ctx, |ui| {
      ui.with_layout(egui::Layout::bottom_up(egui::Align::TOP), |ui| {
        ui.heading("Shield: ");
        ui.heading("Health: ");
      });
    });
}
