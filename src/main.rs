mod level;

use bevy::prelude::*;
use crate::level::read_level;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    read_level("debug")
}