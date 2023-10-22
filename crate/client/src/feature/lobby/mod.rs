#![allow(clippy::module_inception)]
/// Module that defines a lobby.
pub mod lobby;
pub use lobby::*;

mod player;
pub use player::*;

mod scene;
pub use scene::*;

mod camera;
pub use camera::*;

// Reexport stuff from `feature::lobby` module...
// pub use player::PlayerPlugins; // or...
// pub use player::*; // and so on
