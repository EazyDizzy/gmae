#![feature(int_abs_diff)]
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

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
};
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy_fly_camera::FlyCameraPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;

use crate::entity::voxel::Material;
use crate::level::LevelPlugin;
use crate::level::render::init_world;
use crate::system::camera::cursor_grab;

mod system;
mod level;
mod entity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FlyCameraPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LevelPlugin)
        .add_startup_system(system::camera::setup.system())
        // .add_startup_system(system::light::setup.system())
        .add_system(cursor_grab)
        .run();
}