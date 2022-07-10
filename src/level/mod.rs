use bevy::prelude::*;
use lib::entity::level::Level;

use crate::level::reader::read_level;

mod reader;
mod render;

#[allow(clippy::module_name_repetitions)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.get_level_resource())
            .add_startup_system(render::init_world);
    }
}

impl LevelPlugin {
    #[allow(clippy::unused_self)]
    fn get_level_resource(&self) -> Level {
        read_level("debug")
    }
}
