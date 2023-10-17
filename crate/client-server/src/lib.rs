#![allow(clippy::module_inception)]

pub mod feature;
pub mod ui;

/// Pure reusable library modules, except for ui ones go here. Things like physics calculation or little helpers or traits or macros, etc
pub mod lib {
  pub mod api;
  pub mod egui_frame_preset;
  pub mod extend_commands;
  pub mod netutils;
}
