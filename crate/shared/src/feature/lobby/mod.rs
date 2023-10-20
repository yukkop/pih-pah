/// TODO: Module that defines ... (players in the lobby?)
mod player;
pub use player::*;

/// TODO: Module that defines ... (a scene of the lobby?)
mod scene;
pub use scene::*;

pub mod server;

// Reexport stuff from `feature::lobby` module...
// pub use player::PlayerPlugins; // or...
// pub use player::*; // and so on
