use crate::feature::ui::HudPlugins;
use bevy::prelude::*;
use bevy_egui::{
  egui,
  EguiContexts,
};
use bevy::diagnostic::DiagnosticsStore;
use crate::feature::ui::debug::UiDebugState;
use crate::ui::rich_text;
use egui::Align;

use bevy::prelude::*;
pub struct UiPlugins;

use crate::feature::multiplayer::client::InitConnectionEvent;

/// EguiPlugin nessesarly
impl Plugin for UiPlugins {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(HudPlugins)
      .init_resource::<ConnectionState>()
      .init_resource::<UiState>()
      .add_systems(Update, debug_preferences_ui);
  }
}

#[derive(Resource)]
struct UiState {
  is_connection_open: bool,
}

impl Default for UiState {
  fn default() -> Self {
    Self {
      is_connection_open: true,
    }
  }
}

#[derive(Resource)]
struct ConnectionState {
  username: String,
  addr: String
}

impl Default for ConnectionState {
  fn default() -> Self {
    Self {
      username: "noname".to_string(),
      addr: "127.0.0.1:5000".to_string(),
    }
  }
} 

fn debug_preferences_ui(
  mut contexts: EguiContexts,
  mut state: ResMut<ConnectionState>,
  mut ui_state: ResMut<UiState>,
  mut ev: EventWriter<InitConnectionEvent>
) {
  let ctx = contexts.ctx_mut();

  let font = egui::FontId {
    family: egui::FontFamily::Monospace,
    ..default()
  };

  // let screen_center = egui::Pos2 { x: ctx.raw_input().screen_size.x * 0.5, y: ctx.raw_input().screen_size.y * 0.5 };

  if ui_state.is_connection_open {
    egui::Window::new(rich_text("Connection", &font))
      .frame(*crate::lib::egui_frame_preset::NO_SHADOW_FRAME)
      // .default_pos(screen_center)
      .collapsible(false)
      .resizable(false)
      .vscroll(true)
      .show(ctx, |ui| {
        ui.horizontal(|ui| {
          ui.label(rich_text("username", &font));
          ui.add(egui::TextEdit::singleline(&mut state.username));
        });
        ui.horizontal(|ui| {
          ui.label(rich_text("server", &font));
          ui.add(egui::TextEdit::singleline(&mut state.addr));
        });
        if ui.add(egui::Button::new("Connect")).clicked() {
          ev.send(InitConnectionEvent { addr: state.addr.clone(), username: state.username.clone() });
          ui_state.is_connection_open = false;
        }
      });
  }
}

