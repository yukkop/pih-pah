use crate::sound::music::MusicPlugins;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

/// May responsible for all application sound
pub struct SoundPlugins;

impl Plugin for SoundPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((AudioPlugin, MusicPlugins));
    }
}