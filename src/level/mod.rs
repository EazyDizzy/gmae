use bevy::prelude::*;

use crate::entity::voxel::Voxel;
use crate::level::porter::read_level;
use crate::level::render::material::setup as setup_material;

mod render;
mod porter;

#[allow(clippy::module_name_repetitions)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.get_level_resource())
            .add_startup_system(setup_material)
            .add_startup_system_to_stage(StartupStage::PostStartup, render::init_world);
    }
}

impl LevelPlugin {
    fn get_level_resource(&self) -> Level {
        read_level("debug")
    }
}

pub struct Level {
    voxels: Vec<Voxel>,
    day_part: DayPart,
}

#[derive(Copy, Clone, PartialEq)]
pub enum DayPart {
    Day,
    Night,
}

impl Level {
    pub fn new(voxels: Vec<Voxel>, day_part: DayPart) -> Level {
        Level {
            voxels,
            day_part,
        }
    }

    pub fn is_day(&self) -> bool {
        self.day_part == DayPart::Day
    }
}