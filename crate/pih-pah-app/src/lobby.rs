use bevy::{prelude::*, utils::HashMap};
use serde::{Serialize, Deserialize};
use bevy_inspector_egui::prelude::*;

use crate::{controls::PlayerInputs, hashmap};

pub struct LobbyPlugins;

impl Plugin for LobbyPlugins {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Lobby>()
            .register_type::<Lobby>();
    }
}

#[derive(Debug, Clone, PartialEq, Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Lobby {
    pub me: PlayerId,
    pub players: HashMap<PlayerId, Player>,
}

impl Default for Lobby {
    fn default() -> Self {
        Self {
            me: PlayerId::Host,
            players: hashmap!{
                PlayerId::Host => Player::default()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Reflect, Component)]
pub enum PlayerId {
    /// Host or alone
    Host,
    /// Client
    Client(()),
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Reflect, Deserialize)]
pub struct Player {
    /// Client do not need to know about other clients
    #[serde(skip)]
    pub inputs: PlayerInputs,
    pub color: Color,
    #[serde(skip)]
    pub entity: Option<Entity>,
}