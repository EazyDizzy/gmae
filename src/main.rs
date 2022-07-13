#![feature(test)]
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
#[cfg(test)]
extern crate test;

use crate::audio::GameAudioPlugin;
use crate::creature::CreaturePlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use heron::prelude::*;
use lib::entity::voxel::Material;
use lib::util::debug_settings::DebugSettings;
use lib::util::game_settings::GameSettings;
use system::fly_camera::FlyCameraPlugin;

use crate::level::LevelPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UIPlugin;

mod audio;
mod creature;
mod entity;
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
    app.insert_resource(Msaa { samples: 4 })
        .add_state(GameState::Playing)
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        // .add_plugins_with(DefaultPlugins, |plugins| {
        //     plugins.disable::<bevy::log::LogPlugin>()
        // }) // disable LogPlugin so that you can pipe the output directly into `dot -Tsvg`
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
        // .add_plugin(DebugLinesPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(CreaturePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(GameAudioPlugin)
        .add_startup_system(system::light::setup)
        .add_system(game_settings_save);

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

fn game_settings_save(game_settings: ResMut<GameSettings>) {
    if game_settings.is_changed() {
        game_settings.save();
    }
}

#[derive(PhysicsLayer)]
pub enum GamePhysicsLayer {
    World,
    Player,
    Creature,
    Projectile,
}