#![allow(clippy::module_inception)]

mod egui_frame_preset;
mod game_menu;
mod menu;
mod ui;
mod debug;

use egui_frame_preset::*;
pub use debug::*;
pub use game_menu::*;
pub use menu::*;
pub use ui::*;
