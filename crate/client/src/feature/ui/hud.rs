use crate::feature::ui::hud::egui::Align;
use bevy::prelude::*;
use bevy_egui::{
  egui::{self, Color32, Margin, Rounding, Stroke},
  EguiContexts,
};
use epaint::Shadow;
use shared::feature::multiplayer::Lobby;
pub struct HudPlugins;

impl Plugin for HudPlugins {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, ui);
  }
}

fn ui(mut contexts: EguiContexts, lobby: Res<Lobby>) {
  let ctx = contexts.ctx_mut();

  let _transparent_frame = egui::containers::Frame {
    fill: Color32::TRANSPARENT,
    stroke: Stroke::NONE,
    shadow: Shadow::NONE,
    outer_margin: Margin {
      left: 0.,
      right: 0.,
      top: 0.,
      bottom: 0.,
    },
    inner_margin: Margin {
      left: 0.,
      right: 0.,
      top: 0.,
      bottom: 0.,
    },
    rounding: Rounding {
      nw: 0.,
      ne: 0.,
      sw: 0.,
      se: 0.,
    },
  };
  egui::Area::new("right_panel")
    .anchor(egui::Align2::RIGHT_BOTTOM, egui::Vec2::new(0.0, 0.0))
    .show(ctx, |ui| {
      ui.with_layout(egui::Layout::right_to_left(Align::RIGHT), |ui| {
        for (_player_id, player_data) in lobby.players.iter() {
          // TODO Color32
          ui.colored_label(
            Color32::from_rgb(
              (player_data.color.r() * 255.) as u8,
              (player_data.color.g() * 255.) as u8,
              (player_data.color.b() * 255.) as u8,
            ),
            format!("{}", player_data.username),
          );
        }
      });
    });
}
