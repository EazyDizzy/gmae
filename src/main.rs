#![deny(clippy::all, clippy::pedantic, clippy::cognitive_complexity)]
#![allow(
clippy::expect_fun_call,
clippy::cast_sign_loss,
clippy::cast_precision_loss,
clippy::cast_possible_wrap,
clippy::cast_possible_truncation,
clippy::float_cmp,
clippy::default_trait_access,
clippy::needless_pass_by_value,
clippy::module_name_repetitions,
)]

extern crate core;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
};
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy_fly_camera::FlyCameraPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::{AudioPlugin, Audio};
use lib::entity::voxel::Material;
use lib::util::game_settings::GameSettings;

use crate::level::LevelPlugin;
use crate::player::PlayerPlugin;
use crate::system::menu::MenuPlugin;

mod system;
mod level;
mod player;
mod ui;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    Pause,
}

fn main() {
    let settings = GameSettings::from_file("settings.json");

    let mut app = App::new();
    app.add_state(GameState::Playing)
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MenuPlugin)
        .add_system(ui::render)
        .add_startup_system(start_background_audio)
        .add_startup_system(system::light::setup.system());

    if settings.fly_camera {
        app.add_plugin(FlyCameraPlugin);
    }
    if settings.inspector {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    app.insert_resource(settings);

    app.run();
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load("audio/background/forest-birds-chirping-nature-sounds.mp3"));
}