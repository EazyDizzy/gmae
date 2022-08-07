use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::creature::component::hp::HP;
use bevy::math::vec3;
use bevy::prelude::*;
use heron::prelude::*;

use crate::creature::buffs::BuffStorage;
use crate::creature::component::CombatParameters;
use crate::player::animation::{
    animation_rotate_model_on_move, animation_run_on_move, player_animation_setup,
};
use crate::player::attack::{
    player_attack_thrust, player_attack_thrust_check_collisions, ThrustAttackSensor,
};
use crate::player::entity::Player;
use crate::player::system::camera::CameraPlugin;
use crate::player::system::keyboard_interaction::player_track_keyboard_interaction;
use crate::{GamePhysicsLayer, GameState};

mod animation;
mod attack;
pub mod entity;
pub mod system;

#[allow(clippy::module_name_repetitions)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin)
            .add_startup_system(player_setup)
            .add_startup_system(player_animation_setup)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(player_track_keyboard_interaction)
                    .with_system(animation_run_on_move)
                    .with_system(animation_rotate_model_on_move)
                    .with_system(player_attack_thrust)
                    .with_system(player_attack_thrust_check_collisions),
            );
    }
}

pub fn player_setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mesh = asset_server.load("mesh/player.glb#Scene0");
    let comb = CombatParameters::default();
    let phys = PhysiologyDescription::default();

    commands
        .spawn_bundle((
            // TODO take spawn point from world file/save file
            Transform::from_xyz(4., 2., 7.),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(mesh);

            parent
                .spawn_bundle((
                    Transform::from_xyz(0., 0., comb.attack_length / 2. + phys.model_radius),
                    GlobalTransform::identity(),
                ))
                .insert(RigidBody::Sensor)
                .insert(ThrustAttackSensor)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(0.25, 0.5, comb.attack_length / 2.),
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::all_masks::<GamePhysicsLayer>()
                        .with_group(GamePhysicsLayer::Sensor),
                );
        })
        .insert(Player)
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
        .insert(phys)
        .insert(comb)
        // TODO read from save file
        .insert(HP::full(100));
}
