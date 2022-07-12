use crate::creature::component::attack::Attack;
use crate::creature::component::movement::locomotivity::Locomotivity;
use crate::creature::component::movement::MovementStrategy;
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::creature::dummy::Dummy;
use crate::creature::pizza::Pizza;
use crate::player::entity::Player;
use crate::{GamePhysicsLayer, GameState};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use heron::prelude::*;
use heron::rapier_plugin::PhysicsWorld;
use heron::{CollisionLayers, CollisionShape};
use lib::entity::level::creature::CreatureName;
use lib::entity::level::Level;
use lib::entity::point::Point;
use std::f32::consts::PI;

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
                    .with_system(creatures_execute_move_strategies),
            )
            .add_system(creatures_attack_player);
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
            //     TODO remove default rotation after debug
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0))
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
        .insert(CreatureMarker {})
        // .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cylinder {
            radius: 0.5,
            half_height: 1.0,
        })
        .insert(
            CollisionLayers::none()
                .with_group(GamePhysicsLayer::Player)
                .with_mask(GamePhysicsLayer::World),
        );

        if creature.is_enemy() {
            ec.insert(EnemyCreatureMarker {});
        }

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
#[derive(Component, Debug)]
pub struct EnemyCreatureMarker {}

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

fn creatures_attack_player(
    mut lines: ResMut<DebugLines>,
    player_query: Query<&Transform, With<Player>>,
    physics_world: PhysicsWorld,
    mut enemy_query: Query<(
        &Transform,
        &PhysiologyDescription,
        &mut Attack,
        With<EnemyCreatureMarker>,
    )>,
) {
    if let Some(player_transform) = player_query.iter().next() {
        let mut player_position = player_transform.translation;
        player_position.y += 0.5;

        for (transform, phys, mut attack, ..) in enemy_query.iter_mut() {
            let eyes_pos = phys.get_eyes_position(transform).into_vec3();
            let ray_cast_result =
                physics_world.ray_cast(eyes_pos, player_position - eyes_pos, true);

            let can_see = if let Some(cast_info) = ray_cast_result {
                lines.line_colored(eyes_pos, cast_info.collision_point, 0., Color::WHITE);
                player_query.get(cast_info.entity).is_ok()
            } else {
                lines.line_colored(eyes_pos, player_position, 0., Color::WHITE);
                true
            };
        }
    }
}
