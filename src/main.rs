#![deny(clippy::all, clippy::pedantic, clippy::cognitive_complexity)]
#![allow(
clippy::expect_fun_call,
clippy::cast_sign_loss,
clippy::cast_precision_loss,
clippy::cast_possible_wrap,
clippy::cast_possible_truncation,
clippy::float_cmp,
clippy::default_trait_access,
)]

extern crate core;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
};
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy_fly_camera::FlyCameraPlugin;
use lib::entity::voxel::Material;
use lib::util::game_settings::GameSettings;

use crate::level::LevelPlugin;
use crate::system::camera::cursor_grab;

mod system;
mod level;

fn main() {
    let settings = GameSettings::from_file("settings.json");

    App::new()
        .insert_resource(settings)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FlyCameraPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LevelPlugin)
        .add_startup_system(system::camera::setup.system())
        .add_startup_system(system::light::setup.system())
        .add_system(cursor_grab)
        .run();
}