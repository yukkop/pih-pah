use std::time::Duration;
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use bevy_kira_audio::prelude::*;

const MINIMAL_DELAY: f32 = 15.;
const MAXIMAL_DELAY: f32 = 90.;
const MENU_MUSIC_PATH: &str = "lightslategray_blue.wav";

#[derive(Default, Resource)]
struct MusicTimer(Timer);

#[derive(Default, Resource)]
struct MenuMusic(Handle<AudioSource>, Option<Duration>);

pub struct MusicPlugins;

impl Plugin for MusicPlugins {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuMusic>()
            .init_resource::<MusicTimer>()
            .add_systems(Startup, setup)
            .add_systems(Update, play_music);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut menu_music: ResMut<MenuMusic>,
) {
    commands.insert_resource(MusicTimer(Timer::from_seconds(0.0, TimerMode::Repeating)));
    let audio_source: Handle<AudioSource> = asset_server.load(MENU_MUSIC_PATH);
    menu_music.0 = audio_source;
}

fn play_music (
    time: Res<Time>,
    audio: Res<Audio>,
    mut music_timer: ResMut<MusicTimer>,
    mut menu_music: ResMut<MenuMusic>,
    audio_sources: Res<Assets<AudioSource>>,
) {
    if music_timer.0.tick(time.delta()).just_finished() {
        if menu_music.1 == None {
            if let Some(audio_source) = audio_sources.get(&menu_music.0) {
                let duration = audio_source.sound.duration();
                menu_music.1 = Some(duration);
            } else {
              return;
            }
        }

        audio.play(menu_music.0.clone());

        let delay = thread_rng().gen_range(MINIMAL_DELAY..MAXIMAL_DELAY) + menu_music.1.unwrap().as_secs_f32();
        music_timer.0.set_duration(Duration::from_secs_f32(delay));
        music_timer.0.reset();
    }
}