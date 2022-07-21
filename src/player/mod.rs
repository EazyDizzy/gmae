use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::entity::component::hp::HP;
use bevy::math::vec3;
use bevy::prelude::*;
use heron::prelude::*;

use crate::creature::buffs::BuffStorage;
use crate::player::animation::{animation_run_on_move, animation_setup};
use crate::player::entity::Player;
use crate::player::system::camera::CameraPlugin;
use crate::player::system::keyboard_interaction::player_track_keyboard_interaction;
use crate::{GamePhysicsLayer, GameState};

mod animation;
pub mod entity;
mod system;

#[allow(clippy::module_name_repetitions)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin)
            .add_startup_system(setup)
            .add_startup_system(animation_setup)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(player_track_keyboard_interaction)
                    .with_system(animation_run_on_move),
            );
    }
}

pub fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mesh = asset_server.load("mesh/player.glb#Scene0");

    commands
        .spawn_bundle((
            // TODO take spawn point from world file/save file
            Transform::from_xyz(3., 2., 3.),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(mesh);
        })
        .insert(Player::new())
        .insert(BuffStorage::<PhysiologyDescription>::new())
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cylinder {
            radius: 0.5,
            half_height: 1.0,
        })
        .insert(Velocity::from_linear(vec3(0., 0., 0.)))
        .insert(Acceleration::from_linear(Vec3::X * 1.0))
        .insert(RotationConstraints::lock())
        .insert(
            CollisionLayers::all_masks::<GamePhysicsLayer>().with_group(GamePhysicsLayer::Player),
        )
        .insert(PhysiologyDescription::default())
        // TODO read from save file
        .insert(HP::full(100));
}
