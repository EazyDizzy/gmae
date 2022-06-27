use crate::creature::component::moving::Moving;
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::entity::component::hp::HP;
use bevy::math::vec3;
use bevy::prelude::*;
use lib::entity::point::Point;

use crate::player::entity::Player;
use crate::player::system::camera::CameraPlugin;
use crate::player::system::keyboard_interaction::keyboard_interaction;
use crate::GameState;

pub mod entity;
mod system;

#[allow(clippy::module_name_repetitions)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin)
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(keyboard_interaction),
            );
    }
}

pub fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mesh = asset_server.load("mesh/player.glb#Scene0");

    commands
        .spawn_bundle((
            // TODO take spawn point from world file/save file
            Transform::default().with_scale(vec3(0.5, 0.5, 0.5)),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(mesh);
        })
        .insert(Player::new())
        .insert(PhysiologyDescription::default())
        // TODO read from save file
        .insert(Moving::new(Point::new(9.5, 1.0, 3.0)))
        .insert(HP::full(100));
}
