#![allow(clippy::module_inception)]

mod debug;
mod egui_frame_preset;
mod game_menu;
mod menu;
mod runic_inscription;
mod ui;

pub use debug::*;
use egui_frame_preset::*;
pub use game_menu::*;
pub use menu::*;
pub use runic_inscription::*;
pub use ui::*;
