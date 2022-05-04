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
    clippy::module_name_repetitions
)]

extern crate core;

use crate::audio::GameAudioPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_fly_camera::FlyCameraPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use lib::entity::voxel::Material;
use lib::util::debug_settings::DebugSettings;
use lib::util::game_settings::GameSettings;

use crate::level::LevelPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UIPlugin;

mod audio;
mod level;
mod player;
mod system;
mod ui;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    Pause,
}

fn main() {
    let debug_settings = DebugSettings::from_file("debug_settings.json");
    let game_settings = GameSettings::from_file("game_settings.json");

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
        .add_plugin(UIPlugin)
        .add_plugin(GameAudioPlugin)
        .add_startup_system(system::light::setup);

    if debug_settings.fly_camera {
        app.add_plugin(FlyCameraPlugin);
    }
    if debug_settings.inspector {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    app.insert_resource(debug_settings)
        .insert_resource(game_settings);

    app.run();
}
