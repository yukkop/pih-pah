use std::sync::Arc;
use crate::{
  feature::{
    multiplayer::client::InitConnectionEvent,
    ui::HudPlugins,
  },
  ui::rich_text, lib::*
};
use bevy_egui::{
  egui,
  EguiContexts,
};
use bevy::prelude::*;
use epaint::Color32;
use ureq::Error;

#[derive(Resource)]
struct ApiSettings{
  url: Arc<String>,
  token: Option<Arc<String>>,
  me: Option<entity::res::Me>,
}

impl ApiSettings {
  fn new(url: Arc<String>) -> Self {
    Self {
      url,
      token: None,
      me: None,
    }
  }
}

#[derive(Resource)]
struct UiState {
  is_auth_open: bool,
  is_connection_open: bool,
  is_login_open: bool,
  is_register_open: bool,
  is_user_info_open: bool,
}

impl Default for UiState {
  fn default() -> Self {
    Self {
      is_auth_open: true,
      is_connection_open: false,
      is_login_open: false,
      is_register_open: false,
      is_user_info_open: false,
    }
  }
}

#[derive(Default, Resource)]
struct RegisterState {
  username: String,
  account_name: String,
  password: String,
  repeated_password: String,
  password_too_short: bool,
  account_name_too_short: bool,
  username_too_short: bool,
  error_message: Option<Arc<String>>,
}

#[derive(Resource)]
struct LoginState {
  account_name: String,
  password: String,
  error_message: Option<Arc<String>>,
}

impl Default for LoginState {
  fn default() -> Self {
    Self {
      account_name: String::new(),
      password: String::new(),
      error_message: None,
    }
  }
}

#[derive(Resource)]
struct ConnectionState {
  addr: String,
  addresses: Vec::<Arc<String>>,
  error_message: Option<Arc<String>>,
  is_server_not_choisen: bool,
}

impl Default for ConnectionState {
  fn default() -> Self {
    Self {
      addr: "".into(),
      addresses: Vec::<Arc<String>>::new(),
      error_message: None,
      is_server_not_choisen: false,
    }
  }
} 

// Plugin

pub struct UiPlugins {
  api_url: Arc<String>,
}

impl UiPlugins {
  pub fn by_string(api_url: Arc<String>) -> Self {
    Self {
      api_url 
    }
  }
}

/// EguiPlugin nessesarl
impl Plugin for UiPlugins {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ApiSettings::new(format!("http://{}/", *self.api_url).into()))
      .add_plugins(HudPlugins)
      .init_resource::<ConnectionState>()
      .init_resource::<LoginState>()
      .init_resource::<RegisterState>()
      .init_resource::<UiState>()
      .add_systems(Update, (hello, login));
  }
}

fn hello (
  mut contexts: EguiContexts,
  mut ui_state: ResMut<UiState>,
  mut login_state: ResMut<LoginState>,
  mut register_state: ResMut<RegisterState>,
  mut res_api: ResMut<ApiSettings>,
  mut ev: EventWriter<InitConnectionEvent>,
  mut connection_state: ResMut<ConnectionState>,
) {
  let ctx = contexts.ctx_mut();

  let font = egui::FontId {
    family: egui::FontFamily::Monospace,
    ..default()
  };
  // let screen_center = egui::Pos2 { x: ctx.raw_input().screen_size.x * 0.5, y: ctx.raw_input().screen_size.y * 0.5 };

  if ui_state.is_auth_open {
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
            ui_state.is_auth_open = false;
            ui_state.is_login_open = true;
          }
          if ui.add(egui::Button::new("Regester")).clicked() {
            ui_state.is_auth_open = false;
            ui_state.is_register_open = true;
          }
        });
      });
  }

  if ui_state.is_register_open {
    egui::Window::new(rich_text("Register", &font))
      .frame(*crate::lib::egui_frame_preset::NO_SHADOW_FRAME)
      // .default_pos(screen_center)
      .collapsible(false)
      .resizable(false)
      .vscroll(true)
      .show(ctx, |ui| {
        if register_state.error_message.is_some() {
          ui.colored_label(Color32::RED, rich_text(format!("{}", register_state.error_message.clone().unwrap().as_str()), &font));
        }

        // short account name label
        if register_state.account_name_too_short {
          if register_state.account_name.len() > 0 {
            register_state.account_name_too_short = false;
          }
          ui.colored_label(Color32::RED, rich_text("account name too short", &font));
        }
        ui.horizontal(|ui| {
          ui.label(rich_text("account name", &font));
          ui.add(egui::TextEdit::singleline(&mut register_state.account_name));
        });
  
        // short username label
        if register_state.username_too_short {
          if register_state.username.len() > 0 {
            register_state.username_too_short = false;
          }
          ui.colored_label(Color32::RED, rich_text("username too short", &font));
        }
        ui.horizontal(|ui| {
          ui.label(rich_text("username", &font));
          ui.add(egui::TextEdit::singleline(&mut register_state.username));
        });

        // short password label
        if register_state.password_too_short {
          if register_state.password.len() > 0 {
            register_state.password_too_short = false;
          }
          ui.colored_label(Color32::RED, rich_text("password too short", &font));
        }
        ui.horizontal(|ui| {
          ui.label(rich_text("password", &font));
          ui.add(egui::TextEdit::singleline(&mut register_state.password));
        });
        if /* register_state.repeated_password.len() > 0 && */ register_state.repeated_password != register_state.password {
          ui.colored_label(Color32::RED, rich_text("password mismatch", &font));
        }
        ui.horizontal(|ui| {
          ui.label(rich_text("repeated password", &font));
          ui.add(egui::TextEdit::singleline(&mut register_state.repeated_password));
        });

        ui.horizontal(|ui| {
          if ui.add(egui::Button::new("Back")).clicked() {
            ui_state.is_register_open = false;
            ui_state.is_auth_open = true;
          }
          if ui.add(egui::Button::new("Continue")).clicked() {
            if register_state.account_name.len() <= 0 {
              register_state.account_name_too_short = true;
            }
            if register_state.username.len() <= 0 {
              register_state.username_too_short = true;
            }
            if register_state.password.len() <= 0 {
              register_state.password_too_short = true;
            }
            if 
              register_state.repeated_password != register_state.password
              || register_state.password_too_short
              || register_state.username_too_short
              || register_state.account_name_too_short
            {
              return;
            }

            let user = match api::register(
              &res_api.url,
              &register_state.username,
              &register_state.account_name,
              &register_state.password
            ) {
              Ok(user) => {
                let token = api::login(&res_api.url, &register_state.account_name, &register_state.password);
                if token.is_err() {
                  panic!("your api is shit");
                }
                let token = token.ok().expect("your api is shit");
                res_api.token = Some(token.clone());

                let user = api::me(&res_api.url, token.as_ref()).expect("your api is shit");
                res_api.me = Some(user);
                ui_state.is_connection_open = true;
                ui_state.is_login_open = false;
                ui_state.is_register_open = false;
              },
              Err(err) => register_state.error_message = Some(err.message.into()), 
            };
          }
      });
    });
  }

  if ui_state.is_connection_open {
    egui::Window::new(rich_text("Connection", &font))
      // .default_pos(screen_center)
      .collapsible(false)
      .resizable(false)
      .vscroll(false)
      .show(ctx, |ui| {
        if connection_state.error_message.is_some() {
          ui.colored_label(Color32::RED, rich_text(format!("{}", connection_state.error_message.clone().unwrap().as_str()), &font));
        }

        let user = res_api.me.clone().expect("user not exist?");
        ui.horizontal(|ui| {
          ui.label(rich_text(format!("username: {}", user.name.clone()), &font));
        });
        ui.horizontal(|ui| {
          ui.label(rich_text(format!("account name: {}", user.account_name), &font));
        });

        if connection_state.is_server_not_choisen {
          if !connection_state.addr.is_empty() {
            connection_state.is_server_not_choisen = false;
          }
          ui.colored_label(Color32::RED, rich_text("server not choisen", &font));
        }

        // TODO need timer. delay, something
        connection_state.addresses.clear();
        match api::servers(&res_api.url, res_api.token.as_ref().expect("token must be exist here")) {
          Ok(servers) => {
            for server in servers {
              // TODO loader must filter it
              if server.online { 
                connection_state.addresses.push(server.address.into());
              }
            }
          },
          Err(err) => connection_state.error_message = Some(err.message.into()), 
        };

        egui::ComboBox::from_id_source("unique_id")
          .selected_text(&connection_state.addr)
          .show_ui(ui, |ui| {
            for val in &connection_state.addresses.clone() {
              ui.selectable_value(&mut connection_state.addr, val.to_string(), val.to_string());
            } 
          });

        if ui.add(egui::Button::new("Connect")).clicked() {
          if connection_state.addr.is_empty() {
            connection_state.is_server_not_choisen = true;
            return;
          } 
          ev.send(InitConnectionEvent { addr: connection_state.addr.clone(), username: user.name });
          ui_state.is_connection_open = false;
        }
      });
  }
}

fn login(
  mut contexts: EguiContexts,
  mut ui_state: ResMut<UiState>,
  mut login_state: ResMut<LoginState>,
  mut res_api: ResMut<ApiSettings>,
) { 
  let ctx = contexts.ctx_mut();

  let font = egui::FontId {
    family: egui::FontFamily::Monospace,
    ..default()
  };

  if ui_state.is_login_open {
    egui::Window::new(rich_text("Login", &font))
      .frame(*crate::lib::egui_frame_preset::NO_SHADOW_FRAME)
      // .default_pos(screen_center)
      .collapsible(false)
      .resizable(false)
      .vscroll(false)
      .show(ctx, |ui| {
        if login_state.error_message.is_some() {
          ui.colored_label(Color32::RED, rich_text(format!("{}", login_state.error_message.clone().unwrap().as_str()), &font));
        }
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
            ui_state.is_login_open = false;
            ui_state.is_auth_open = true;
          }
          if ui.add(egui::Button::new("Continue")).clicked() {
            let token = match api::login(&res_api.url, &login_state.account_name, &login_state.password) {
              Ok(token) => {
                res_api.token = Some(token.clone());

                match api::me(&res_api.url, token.as_ref()) {
                  Ok(user) => {
                    res_api.me = Some(user);
                    ui_state.is_connection_open = true;
                    ui_state.is_login_open = false;
                  },
                  Err(err) => login_state.error_message = Some(err.message.into()), 
                };
              }, 
              Err(err) => login_state.error_message = Some(err.message.into()),
            };
          }
        });
      });
  }
}

// fn misc(
//   mut contexts: EguiContexts,
//   mut ui_state: ResMut<UiState>,
// ) {
//   let ctx = contexts.ctx_mut();
//
//   let font = egui::FontId {
//     family: egui::FontFamily::Monospace,
//     ..default()
//   };
//
// }
