use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::sound::music::MusicPlugins;

pub struct SoundPlugins;

impl Plugin for SoundPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((AudioPlugin, MusicPlugins));
    }
}