use crate::creature::component::movement::locomotivity::Locomotivity;
use crate::creature::component::movement::MovementStrategy;
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::creature::dummy::Dummy;
use crate::entity::component::hp::HP;
use crate::GameState;
use bevy::math::vec3;
use bevy::prelude::*;
use lib::entity::level::Level;
use lib::entity::point::Point;

pub mod component;
pub mod dummy;

#[allow(clippy::module_name_repetitions)]
pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_creatures)
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_creatures));
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
                ),
                GlobalTransform::identity(),
            ))
            .with_children(|parent| {
                parent.spawn_scene(dummy_mesh.clone());
            })
            .insert(Dummy::new())
            .insert(Creature {})
            .insert(MovementStrategy::random())
            .insert(PhysiologyDescription::default())
            .insert(Locomotivity::new(Point::new(
                creature.position.x,
                creature.position.y,
                creature.position.z,
            )))
            .insert(HP::full(100));
    }
}

#[derive(Component, Debug)]
pub struct Creature {}

fn move_creatures(
    lvl: Res<Level>,
    mut query: Query<(
        &mut Transform,
        &mut Locomotivity,
        &PhysiologyDescription,
        &mut MovementStrategy,
        With<Creature>,
    )>,
) {
    for (mut transform, mut locomotivity, phys, mut move_strat, ..) in query.iter_mut() {
        move_strat.update(&mut locomotivity, phys, &lvl);
        transform.translation = vec3(
            locomotivity.position().x,
            locomotivity.position().y,
            locomotivity.position().z,
        );
    }
}