#![allow(clippy::module_inception)]
/// Module that defines a lobby.
mod lobby;
pub use lobby::*;

mod player;
pub use player::*;

mod scene;
pub use scene::*;
