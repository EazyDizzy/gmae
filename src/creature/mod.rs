use crate::creature::dummy::Dummy;
use crate::entity::component::hp::HP;
use bevy::math::vec3;
use bevy::prelude::*;
use lib::entity::level::Level;

pub mod dummy;
pub mod component;

#[allow(clippy::module_name_repetitions)]
pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_creatures);
    }
}

fn spawn_creatures(mut commands: Commands, level: Res<Level>, asset_server: Res<AssetServer>) {
    let dummy_mesh = asset_server.load("mesh/dummy.glb#Scene0");

    for creature in level.creatures() {
        commands
            .spawn_bundle((
                Transform::from_xyz(
                    creature.position.x,
                    creature.position.y,
                    creature.position.z,
                )
                .with_scale(vec3(0.5, 0.5, 0.5)),
                GlobalTransform::identity(),
            ))
            .with_children(|parent| {
                parent.spawn_scene(dummy_mesh.clone());
            })
            .insert(Dummy::new())
            .insert(Creature {})
            .insert(HP::full(100));
    }
}

#[derive(Component, Debug)]
pub struct Creature {}
