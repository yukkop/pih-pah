use crate::feature::ui::HudPlugins;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use bevy::tasks::TaskPool;
use bevy_egui::{
  egui,
  EguiContexts,
};
use bevy::diagnostic::DiagnosticsStore;
use crate::{
  feature::{
    multiplayer::client::InitConnectionEvent,
    ui::debug::UiDebugState
  },
  ui::rich_text
};
use egui::Align;

use bevy::prelude::*;
pub struct UiPlugins;


/// EguiPlugin nessesarly
impl Plugin for UiPlugins {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(HudPlugins)
      .init_resource::<ConnectionState>()
      .init_resource::<LoginState>()
      .init_resource::<RegisterState>()
      .init_resource::<UiState>()
      .add_systems(Update, debug_preferences_ui);
  }
}

#[derive(Resource)]
struct UiState {
  is_hello: bool,
  is_connection_open: bool,
  is_login: bool,
  is_register: bool,
}

impl Default for UiState {
  fn default() -> Self {
    Self {
      is_hello: true,
      is_connection_open: false,
      is_login: false,
      is_register: false,
    }
  }
}


#[derive(Default, Resource)]
struct RegisterState {
  account_name: String,
  password: String,
  repeated_password: String,
}

#[derive(Default, Resource)]
struct LoginState {
  account_name: String,
  password: String,
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
  mut connection_state: ResMut<ConnectionState>,
  mut register_state: ResMut<RegisterState>,
  mut ui_state: ResMut<UiState>,
  mut login_state: ResMut<LoginState>,
  mut ev: EventWriter<InitConnectionEvent>,
) {
  let ctx = contexts.ctx_mut();

  let font = egui::FontId {
    family: egui::FontFamily::Monospace,
    ..default()
  };

  // let screen_center = egui::Pos2 { x: ctx.raw_input().screen_size.x * 0.5, y: ctx.raw_input().screen_size.y * 0.5 };

  if ui_state.is_hello {
    egui::Window::new(rich_text("Hello", &font))
      .frame(*crate::lib::egui_frame_preset::NO_SHADOW_FRAME)
      // .default_pos(screen_center)
      .collapsible(false)
      .resizable(false)
      .vscroll(true)
      .show(ctx, |ui| {
        ui.label(rich_text("Am I seeing you for the first time?", &font));
        ui.horizontal(|ui| {
          if ui.add(egui::Button::new("Login")).clicked() {
            ui_state.is_hello = false;
            ui_state.is_login = true;
          }
          if ui.add(egui::Button::new("Regester")).clicked() {
            ui_state.is_hello = false;
            ui_state.is_register = true;
          }
          // TODO remove
          if ui.add(egui::Button::new("не хочу ждать")).clicked() {
            ui_state.is_hello = false;
            ui_state.is_connection_open = true;
          }
        });
      });
  }

  if ui_state.is_register {
    egui::Window::new(rich_text("Login", &font))
      .frame(*crate::lib::egui_frame_preset::NO_SHADOW_FRAME)
      // .default_pos(screen_center)
      .collapsible(false)
      .resizable(false)
      .vscroll(true)
      .show(ctx, |ui| {
        ui.horizontal(|ui| {
          ui.label(rich_text("account name", &font));
          ui.add(egui::TextEdit::singleline(&mut register_state.account_name));
        });
        ui.horizontal(|ui| {
          ui.label(rich_text("password", &font));
          ui.add(egui::TextEdit::singleline(&mut register_state.password));
        });
        ui.horizontal(|ui| {
          ui.label(rich_text("password", &font));
          ui.add(egui::TextEdit::singleline(&mut register_state.repeated_password));
        });

        ui.horizontal(|ui| {
          if ui.add(egui::Button::new("Back")).clicked() {
            ui_state.is_register = false;
            ui_state.is_hello = true;
          }
          if ui.add(egui::Button::new("Continue")).clicked() {
            ui_state.is_register = false;
          }
        });
      });
  }
  use serde_json::json;
  use ureq::Error;

  if ui_state.is_login {
    egui::Window::new(rich_text("Login", &font))
      .frame(*crate::lib::egui_frame_preset::NO_SHADOW_FRAME)
      // .default_pos(screen_center)
      .collapsible(false)
      .resizable(false)
      .vscroll(true)
      .show(ctx, |ui| {
        ui.horizontal(|ui| {
          ui.label(rich_text("account name", &font));
          ui.add(egui::TextEdit::singleline(&mut login_state.account_name));
        });
        ui.horizontal(|ui| {
          ui.label(rich_text("password", &font));
          ui.add(egui::TextEdit::singleline(&mut login_state.password));
        });
        ui.horizontal(|ui| {
          if ui.add(egui::Button::new("Back")).clicked() {
            ui_state.is_login = false;
            ui_state.is_hello = true;
          }
          if ui.add(egui::Button::new("Continue")).clicked() {
            ui_state.is_login = false;
              AsyncComputeTaskPool::get()
                .spawn(async move {
                  let url = "http://127.0.0.1:8000/user/login";
                  let json_body = json!({
                      "account_name": "tosh_uniq",
                      "password": "123"
                  });
                  
                  let resp = ureq::post(url)
                      .set("Content-Type", "application/json")
                      .send_json(json_body);

                  match resp {
                      Ok(kek) => log::info!("{}", kek.into_string().unwrap()),
                      Err(Error::Status(code, response)) => log::error!("Error: {}, {:#?}", code, response),
                      Err(_) => { log::error!("Oaoaoaoaoaoaaoaoao") }
                  };
                })
                .detach();
          }
        });
      });
  }

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
          ui.add(egui::TextEdit::singleline(&mut connection_state.username));
        });
        ui.horizontal(|ui| {
          ui.label(rich_text("server", &font));
          ui.add(egui::TextEdit::singleline(&mut connection_state.addr));
        });
        if ui.add(egui::Button::new("Connect")).clicked() {
          ev.send(InitConnectionEvent { addr: connection_state.addr.clone(), username: connection_state.username.clone() });
          ui_state.is_connection_open = false;
        }
      });
  }

}
