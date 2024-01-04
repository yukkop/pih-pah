pub mod game;
pub mod ui;
pub mod util;
pub mod option;
pub mod sound;
#[cfg(feature = "dev")]
pub mod editor;
pub mod level_editor;
pub mod controls;
pub mod asset_loader;
pub mod lobby;

/// The directory where the assets are located
pub const ASSET_DIR: &str = "asset";

