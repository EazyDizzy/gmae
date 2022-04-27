use bevy::math::vec3;
use bevy::prelude::*;

use crate::GameState;
use crate::player::entity::Player;
use crate::player::system::camera::CameraPlugin;
use crate::player::system::keyboard_interaction::keyboard_interaction;

mod entity;
mod system;

#[allow(clippy::module_name_repetitions)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(CameraPlugin)
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(keyboard_interaction.system()),
            )
        ;
    }
}

pub fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mesh = asset_server.load("mesh/player.glb#Scene0");

    commands.spawn_bundle((
        Transform::default()
            .with_scale(vec3(0.5, 0.5, 0.5)),
        GlobalTransform::identity(),
    )).with_children(|parent| {
        parent.spawn_scene(mesh);
    })
        .insert(Player::new());
}

