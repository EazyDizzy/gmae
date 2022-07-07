use bevy::prelude::*;
use lib::entity::level::Level;

use crate::level::reader::read_level;
use crate::level::render::material::setup as setup_material;
use crate::level::render::named_materials::NamedMaterials;

mod reader;
mod render;

#[allow(clippy::module_name_repetitions)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.get_level_resource())
            .insert_resource(NamedMaterials::empty())
            .add_startup_system(setup_material)
            .add_startup_system_to_stage(StartupStage::PostStartup, render::init_world);
    }
}

impl LevelPlugin {
    #[allow(clippy::unused_self)]
    fn get_level_resource(&self) -> Level {
        read_level("bug_detection")
    }
}
