use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_kira_audio::{Audio, AudioControl};
use lib::util::game_settings::GameSettings;
use rand::Rng;
use std::collections::HashMap;

pub struct GameAudioPlugin;
pub struct Sounds {
    damage: HashMap<DamageSoundType, Vec<Handle<AudioSource>>>,
    #[allow(dead_code)]
    background: Vec<Handle<AudioSource>>,
}
pub struct SoundEvent {
    pub sound_layer: SoundLayer,
    pub sound_type: SoundType,
}
#[derive(Copy, Clone)]
pub enum SoundType {
    Damage(DamageSoundType),
}
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum DamageSoundType {
    Punch,
    Bullet,
}
#[derive(Copy, Clone)]
pub enum SoundLayer {
    ForeGround,
    #[allow(dead_code)]
    Background,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>()
            .add_system(audio_adjust_volume_from_settings)
            .add_system(audio_play_sound_events)
            .add_startup_system(audio_setup_sounds)
            // .add_startup_system(start_background_audio)
        ;
    }
}

fn audio_setup_sounds(asset_server: Res<AssetServer>, mut commands: Commands) {
    let load = |path: &str| -> Handle<AudioSource> { asset_server.load(path) };
    let mut damage = HashMap::new();

    damage.insert(
        DamageSoundType::Punch,
        vec![
            load("audio/foreground/damage/punch/0.mp3"),
            load("audio/foreground/damage/punch/1.mp3"),
            load("audio/foreground/damage/punch/2.mp3"),
            load("audio/foreground/damage/punch/3.mp3"),
            load("audio/foreground/damage/punch/4.mp3"),
            load("audio/foreground/damage/punch/5.mp3"),
        ],
    );
    damage.insert(
        DamageSoundType::Bullet,
        vec![
            load("audio/foreground/damage/bullet/body_0.mp3"),
            load("audio/foreground/damage/bullet/body_1.mp3"),
        ],
    );

    commands.insert_resource(Sounds {
        damage,
        // for future music switching according to location
        background: vec![load(
            "audio/background/forest-birds-chirping-nature-sounds.mp3",
        )],
    });
}
#[allow(dead_code)]
fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let b = asset_server.load("audio/background/forest-birds-chirping-nature-sounds.mp3");
    audio.play(b).looped();
}

fn audio_adjust_volume_from_settings(audio: Res<Audio>, game_settings: Res<GameSettings>) {
    if game_settings.is_changed() {
        audio.set_volume(game_settings.get_background_music_volume());
    }
}

fn audio_play_sound_events(
    mut sound_events: EventReader<SoundEvent>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
) {
    let mut rng = rand::thread_rng();

    for event in sound_events.iter() {
        match event.sound_type {
            SoundType::Damage(source) => {
                if let Some(sounds) = sounds.damage.get(&source) {
                    let hit_sound = rng.gen_range(0..sounds.len());
                    audio.play(sounds[hit_sound].clone());
                }
            }
        }
    }
}
