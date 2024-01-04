//! This layer eases the transition between UI crates.
//! Replace the current plugin call with the desired one.
//!
//! start game with all ui features is not good idea
//!
//! I'm expect use `egui` for ui prototype and `lunex` for prodaction game ui

// Ensure only one UI crate feature is chosen: `lunex` or `egui`
#[cfg(not(any(feature = "ui_lunex", feature = "ui_egui")))]
compile_error!("Select one UI crate feature: `ui_lunex` or `ui_egui`.");

#[cfg(feature = "ui_lunex")]
mod lunex;

use bevy::app::{App, Plugin};

// Make LunexPlugins internal to prevent public dependencies on UI crate specifics
#[cfg(feature = "ui_lunex")]
use lunex::LunexPlugins;

// Define other plugins if necessary
// TODO #[cfg(feature = "ui_egui")]
mod egui;

#[cfg(feature = "ui_egui")]
use self::egui::EguiPlugins;

mod logic;
pub use logic::*;

pub struct MenuPlugins;

impl Plugin for MenuPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiLogicPlugins);

        // Add the chosen plugin based on the feature
        #[cfg(feature = "ui_lunex")]
        app.add_plugins(LunexPlugins);

        #[cfg(feature = "ui_egui")]
        app.add_plugins(EguiPlugins);
    }
}
