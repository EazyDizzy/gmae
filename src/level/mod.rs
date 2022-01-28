use bevy::prelude::*;

use crate::level::render::material::setup as setup_material;
use crate::level::render::mesh::setup as setup_mesh;
use crate::render_world;

pub mod render;
mod porter;
mod util;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_mesh)
            .add_startup_system(setup_material)
            .add_startup_system(render_world);
    }
}
