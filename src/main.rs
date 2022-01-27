use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
};
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy_fly_camera::FlyCameraPlugin;

use crate::entity::voxel::VoxelMaterial;
use crate::level::read_level;
use crate::level::render::render_world;
use crate::system::camera::{cursor_grab, initial_grab_cursor};
use crate::system::light::{spawn_blue_light_source, spawn_orange_light_source};

mod system;
mod level;
mod entity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(render_world)
        .add_startup_system(system::camera::setup.system())
        .add_startup_system(initial_grab_cursor)
        .add_system(cursor_grab)
        .run();
}