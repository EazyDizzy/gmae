use crate::creature::component::movement::locomotivity::Locomotivity;
use crate::creature::component::movement::{MovementStrategy, CREATURE_MOVED_LABEL};
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::creature::dummy::Dummy;
use crate::creature::pizza::Pizza;
use crate::GameState;
use bevy::math::vec3;
use bevy::prelude::*;
use lib::entity::level::creature::CreatureName;
use lib::entity::level::Level;

pub mod component;
pub mod dummy;
pub mod pizza;

#[allow(clippy::module_name_repetitions)]
pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_creatures)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(creatures_execute_move_strategies)
                    .label(CREATURE_MOVED_LABEL),
            )
            .add_system(
                creatures_apply_gravity
                    .after(CREATURE_MOVED_LABEL)
                    .before(creature_move_model),
            )
            .add_system(creature_move_model.after(CREATURE_MOVED_LABEL));
    }
}

fn spawn_creatures(mut commands: Commands, level: Res<Level>, asset_server: Res<AssetServer>) {
    let dummy_mesh = asset_server.load("mesh/dummy.glb#Scene0");
    let pizza_mesh = asset_server.load("mesh/pizza.glb#Scene0");

    for creature in level.creatures() {
        let mut ec = commands.spawn_bundle((
            Transform::from_xyz(
                creature.position.x,
                creature.position.y,
                creature.position.z,
            )
            //     TODO make sth to avoid this
            .with_scale(vec3(0.5, 0.5, 0.5)),
            GlobalTransform::identity(),
        ));
        ec.with_children(|parent| {
            let mesh = match creature.name {
                CreatureName::Dummy => dummy_mesh.clone(),
                CreatureName::Pizza => pizza_mesh.clone(),
            };
            parent.spawn_scene(mesh);
        })
        .insert(CreatureMarker {});

        match creature.name {
            CreatureName::Dummy => {
                ec.insert(Dummy::new());
                dummy::insert(&mut ec, creature);
            }
            CreatureName::Pizza => {
                ec.insert(Pizza::new());
                pizza::insert(&mut ec, creature);
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct CreatureMarker {}

fn creatures_execute_move_strategies(
    lvl: Res<Level>,
    mut query: Query<(
        &mut Locomotivity,
        &PhysiologyDescription,
        &mut MovementStrategy,
        With<CreatureMarker>,
    )>,
) {
    for (mut locomotivity, phys, mut move_strat, ..) in query.iter_mut() {
        move_strat.update(&mut locomotivity, phys, &lvl);
    }
}

fn creature_move_model(mut query: Query<(&mut Transform, &Locomotivity)>) {
    for (mut transform, locomotivity) in query.iter_mut() {
        transform.translation = vec3(
            locomotivity.position().x,
            locomotivity.position().y,
            locomotivity.position().z,
        );
    }
}

fn creatures_apply_gravity(
    lvl: Res<Level>,
    mut query: Query<(&mut Locomotivity, &PhysiologyDescription)>,
) {
    for (mut locomotivity, phys) in query.iter_mut() {
        locomotivity.gravity_move(&lvl, phys);
    }
}
