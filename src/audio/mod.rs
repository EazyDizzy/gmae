use bevy::prelude::*;
use bevy_kira_audio::Audio;
use lib::util::game_settings::GameSettings;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(adjust_volume)
            .add_startup_system(start_background_audio);
    }
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play_looped(asset_server.load("audio/background/forest-birds-chirping-nature-sounds.mp3"));
}

fn adjust_volume(audio: Res<Audio>, game_settings: Res<GameSettings>) {
    audio.set_volume(game_settings.get_background_music_volume() as f32);
}