use crate::creature::component::attack::component::Attack;
use crate::creature::component::attack::AttackPlugin;
use crate::creature::component::movement::MovementStrategy;
use crate::creature::component::physiology_description::PhysiologyDescription;

use crate::creature::buffs::BuffsPlugin;
use crate::creature::component::hp::{creature_hp_mesh_change_percent, creature_hp_spawn_mesh};
use crate::creature::mob::{dummy, pizza};
use crate::player::PlayerMarker;
use crate::{GamePhysicsLayer, GameState};
use bevy::math::vec3;
use bevy::prelude::*;
use heron::prelude::*;
use heron::rapier_plugin::PhysicsWorld;
use heron::{CollisionLayers, CollisionShape};
use lib::entity::level::creature::CreatureName;
use lib::entity::level::Level;
use std::f32::consts::PI;

pub mod buffs;
pub mod component;
pub mod mob;

#[derive(Component, Debug)]
pub struct CreatureMarker;
#[derive(Component, Debug)]
pub struct EnemyCreatureMarker;

#[allow(clippy::module_name_repetitions)]
pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AttackPlugin)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_creatures)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(creature_execute_move_strategies)
                    .with_system(creature_attack_player)
                    .with_system(creature_hp_mesh_change_percent)
                    .with_system(creature_hp_spawn_mesh),
            )
            .add_plugin(BuffsPlugin);
    }
}

fn spawn_creatures(mut commands: Commands, level: Res<Level>, asset_server: Res<AssetServer>) {
    for creature in level.creatures() {
        let mut ec = commands.spawn_bundle(SceneBundle {
            scene: match creature.name {
                CreatureName::Dummy => asset_server.load("mesh/dummy.glb#Scene0"),
                CreatureName::Pizza => asset_server.load("mesh/pizza.glb#Scene0"),
            },
            transform: Transform::from_xyz(
                creature.position.x,
                creature.position.y + 0.5, // To prevent stucking in the ground
                creature.position.z,
            )
            //     TODO remove default rotation after debug
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0))
            //     TODO make sth to avoid this
            .with_scale(vec3(0.5, 0.5, 0.5)),
            ..Default::default()
        });
        ec.insert(CreatureMarker)
            .insert(RigidBody::Dynamic)
            .insert(RotationConstraints::lock())
            .insert(CollisionShape::Cylinder {
                radius: 0.5,
                half_height: 1.0,
            })
            .insert(
                CollisionLayers::all_masks::<GamePhysicsLayer>()
                    .with_group(GamePhysicsLayer::Creature),
            )
            .insert(EnemyCreatureMarker);

        match creature.name {
            CreatureName::Dummy => {
                dummy::insert(&mut ec);
            }
            CreatureName::Pizza => {
                pizza::insert(&mut ec);
            }
        }
    }
}

fn creature_execute_move_strategies(
    lvl: Res<Level>,
    mut query: Query<
        (
            &PhysiologyDescription,
            &mut MovementStrategy,
            &Transform,
            &mut Velocity,
        ),
        With<CreatureMarker>,
    >,
) {
    for (phys, mut move_strat, transform, mut velocity) in query.iter_mut() {
        move_strat.update(phys, &lvl, transform, &mut velocity);
    }
}

fn creature_attack_player(
    player_query: Query<(Entity, &Transform), With<PlayerMarker>>,
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
