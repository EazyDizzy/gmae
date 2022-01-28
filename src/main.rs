use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
};
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy_fly_camera::FlyCameraPlugin;

use crate::entity::voxel::VoxelMaterial;
use crate::level::LevelPlugin;
use crate::level::render::render_world;
use crate::system::camera::cursor_grab;

mod system;
mod level;
mod entity;
const ENABLE_EXTREME_GRAPHIC: bool = false;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FlyCameraPlugin)
        .add_plugin(LevelPlugin)
        .add_startup_system(system::camera::setup.system())
        .add_startup_system(system::light::setup.system())
        .add_system(cursor_grab)
        .run();
}