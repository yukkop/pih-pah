use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct MusicPlugins;

impl Plugin for MusicPlugins {
    fn build(&self, app: &mut App) {
        app
          .add_plugins(AudioPlugin)
          .add_systems(Startup, setup);
    }
}

fn setup(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("lightslategray_blue.wav")).looped();
}
