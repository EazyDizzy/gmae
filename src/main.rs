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
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use lib::entity::voxel::Material;
use lib::util::debug_settings::DebugSettings;
use lib::util::game_settings::GameSettings;
use system::fly_camera::FlyCameraPlugin;

use crate::level::LevelPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UIPlugin;

mod audio;
mod level;
mod player;
mod system;
mod ui;
mod entity;

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
        // .add_plugins_with(DefaultPlugins, |plugins| {
        //     plugins.disable::<bevy::log::LogPlugin>()
        // }) // disable LogPlugin so that you can pipe the output directly into `dot -Tsvg`
        // .add_plugin(ConfigCam)
        // .insert_resource(MovementSettings {
        //     sensitivity: 0.00015, // default: 0.00012
        //     speed: 15.0,          // default: 12.0
        //     dist: 5.0,            // Camera distance from the player in topdown view
        //     ..Default::default()
        // })
        // .insert_resource(PlayerSettings {
        //     pos: Vec3::new(2., 0., 2.), //Initial position of the player
        //     ..Default::default()
        // })
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

    // bevy_mod_debugdump::print_schedule(&mut app);
    // bevy_mod_debugdump::print_render_graph(&mut app);
    // bevy_mod_debugdump::print_render_schedule(&mut app);
    app.run();
}
