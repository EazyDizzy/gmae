use bevy::prelude::*;

use crate::level::render::material::setup as setup_material;
use crate::init_world;

pub mod render;
mod porter;
mod util;

#[allow(clippy::module_name_repetitions)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_material)
            .add_startup_system_to_stage(StartupStage::PostStartup, init_world);
    }
}
