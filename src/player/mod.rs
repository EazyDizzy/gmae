use std::f32::consts::PI;
use bevy::prelude::*;

use crate::player::entity::Player;

mod entity;

#[allow(clippy::module_name_repetitions)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup);
    }
}

pub fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mesh = asset_server.load("mesh/player/source/kong.glb#Scene0");

    let id = commands.spawn_bundle((
        Transform::from_xyz(3.0, 3.0, -0.5)
            .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
        GlobalTransform::identity(),
    )).with_children(|parent| {
        parent.spawn_scene(mesh);
    }).id();

    commands.insert_resource(Player::new(id));
}

