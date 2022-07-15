use crate::creature::component::attack::component::Attack;
use crate::creature::component::attack::AttackPlugin;
use crate::creature::component::movement::locomotivity::Locomotivity;
use crate::creature::component::movement::MovementStrategy;
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::creature::dummy::Dummy;
use crate::creature::pizza::Pizza;
use crate::player::entity::Player;
use crate::{GamePhysicsLayer, GameState};
use bevy::math::vec3;
use bevy::prelude::*;
use heron::prelude::*;
use heron::rapier_plugin::PhysicsWorld;
use heron::{CollisionLayers, CollisionShape};
use lib::entity::level::creature::CreatureName;
use lib::entity::level::Level;
use std::f32::consts::PI;
use lib::entity::point::Point;
use crate::creature::buffs::BuffsPlugin;

pub mod component;
pub mod dummy;
pub mod buffs;
pub mod pizza;

#[allow(clippy::module_name_repetitions)]
pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AttackPlugin)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_creatures)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(creatures_execute_move_strategies)
                    .with_system(creatures_attack_player),
            )
            .add_plugin(BuffsPlugin);
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
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(CollisionShape::Cylinder {
            radius: 0.5,
            half_height: 1.0,
        })
        .insert(
            CollisionLayers::none()
                .with_group(GamePhysicsLayer::Creature)
                .with_masks([GamePhysicsLayer::World, GamePhysicsLayer::Player]),
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
    mut query: Query<
        (
            &mut Locomotivity,
            &PhysiologyDescription,
            &mut MovementStrategy,
            &Transform,
            &mut Velocity,
        ),
        With<CreatureMarker>,
    >,
) {
    for (mut locomotivity, phys, mut move_strat, transform, mut velocity) in query.iter_mut() {
        move_strat.update(&mut locomotivity, phys, &lvl, transform, &mut velocity);
    }
}

fn creatures_attack_player(
    player_query: Query<(Entity, &Transform), With<Player>>,
    physics_world: PhysicsWorld,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut enemy_query: Query<
        (&Transform, &PhysiologyDescription, &mut Attack),
        With<EnemyCreatureMarker>,
    >,
) {
    if let Some((id, player_transform)) = player_query.iter().next() {
        let mut player_position = player_transform.translation;
        player_position.y += 0.5;

        for (transform, phys, mut attack) in enemy_query.iter_mut() {
            attack.exec(
                &physics_world,
                phys,
                transform,
                player_position,
                id,
                &mut commands,
                &mut meshes,
            );
        }
    }
}
