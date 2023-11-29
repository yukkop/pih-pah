use std::time::Duration;
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use bevy_kira_audio::prelude::*;

const MINIMAL_DELAY: f32 = 15.;
const MAXIMAL_DELAY: f32 = 90.;

#[derive(Default, Resource)]
struct MusicTimer(Timer);

pub struct MusicPlugins;

impl Plugin for MusicPlugins {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MusicTimer>()
            .add_systems(Startup, setup)
            .add_systems(Update, play_music);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(MusicTimer(Timer::from_seconds(0.0, TimerMode::Repeating)));
}

fn play_music(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut music_timer: ResMut<MusicTimer>,
) {
    if music_timer.0.tick(time.delta()).just_finished() {
        audio.play(asset_server.load("lightslategray_blue.wav"));

        let delay = thread_rng().gen_range(MINIMAL_DELAY..MAXIMAL_DELAY);
        music_timer.0.set_duration(Duration::from_secs_f32(delay));
        music_timer.0.reset();
    }
}