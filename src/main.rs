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
)]

extern crate core;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
};
use bevy::diagnostic::LogDiagnosticsPlugin;
use lib::entity::voxel::Material;
use lib::util::game_settings::GameSettings;

use crate::level::LevelPlugin;
use crate::player::PlayerPlugin;
use crate::system::menu::MenuPlugin;

mod system;
mod level;
mod player;
mod util;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
}

fn main() {
    let settings = GameSettings::from_file("settings.json");

    App::new()
        .insert_resource(settings)
        .add_state(GameState::Playing)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FlyCameraPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MenuPlugin)
        .add_startup_system(system::light::setup.system())
        .run();
}