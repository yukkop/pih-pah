use crate::feature::ui::HudPlugins;
use bevy::prelude::*;
use bevy_egui::{
  egui,
  EguiContexts,
};
use bevy::diagnostic::DiagnosticsStore;
use crate::feature::ui::debug::UiDebugState;
use crate::ui::rich_text;

use bevy::prelude::*;
pub struct UiPlugins;

use crate::feature::multiplayer::client::InitConnectionEvent;

/// EguiPlugin nessesarly
impl Plugin for UiPlugins {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(HudPlugins)
      .init_resource::<ConnectionState>()
      .add_systems(Update, debug_preferences_ui);
  }
}

#[derive(Resource)]
struct ConnectionState{username: String}

impl Default for ConnectionState {
  fn default() -> Self {
    Self {
      username: "noname".to_string()
    }
  }
} 

fn debug_preferences_ui(
  mut contexts: EguiContexts,
  mut state: ResMut<ConnectionState>,
  mut ev: EventWriter<InitConnectionEvent>
) {
  let ctx = contexts.ctx_mut();

  let font = egui::FontId {
    family: egui::FontFamily::Monospace,
    ..default()
  };

  egui::Window::new(rich_text("Connection", &font))
    .frame(*crate::lib::egui_frame_preset::NO_SHADOW_FRAME)
    .vscroll(true)
    .show(ctx, |ui| {
      ui.label(rich_text("ur name", &font));
      ui.add(egui::TextEdit::singleline(&mut state.username));
      if ui.add(egui::Button::new("Connect")).clicked() {
        ev.send(InitConnectionEvent { addr: "127.0.0.1:5000".to_string(), username: state.username.clone() });
      }
    });
}

