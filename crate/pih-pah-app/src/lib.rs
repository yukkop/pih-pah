pub mod asset_loader;
pub mod controls;
#[cfg(feature = "dev")]
pub mod editor;
pub mod game;
pub mod level_editor;
pub mod lobby;
pub mod option;
pub mod sound;
pub mod ui;
pub mod util;

/// The directory where the assets are located
pub const ASSET_DIR: &str = "asset";
