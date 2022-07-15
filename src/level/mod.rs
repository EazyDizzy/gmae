use crate::GamePhysicsLayer;
use bevy::prelude::*;
use heron::{CollisionEvent, CollisionLayers, CollisionShape, RigidBody};
use lib::entity::level::Level;

use crate::level::reader::read_level;

mod reader;
mod render;

#[allow(clippy::module_name_repetitions)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.get_level_resource())
            .add_startup_system(render::level_init)
            .add_startup_system(level_spawn_killing_sensor)
            .add_system(level_kill_entities_on_sensor_touch);
    }
}

impl LevelPlugin {
    #[allow(clippy::unused_self)]
    fn get_level_resource(&self) -> Level {
        read_level("debug")
    }
}

#[derive(Component)]
struct KillingSensor;

fn level_spawn_killing_sensor(mut commands: Commands) {
    commands
        .spawn_bundle((
            Transform::from_xyz(0., -5., 0.),
            GlobalTransform::identity(),
        ))
        .insert(RigidBody::Sensor)
        .insert(KillingSensor)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(999., 1., 999.),
            border_radius: None,
        })
        .insert(
            CollisionLayers::all_masks::<GamePhysicsLayer>().with_group(GamePhysicsLayer::Sensor),
        );
}

fn level_kill_entities_on_sensor_touch(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
) {
    collision_events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            let with_sensor = is_sensor(layers_1) || is_sensor(layers_2);

            if with_sensor {
                return if is_sensor(layers_1) {
                    Some(entity_2)
                } else {
                    Some(entity_1)
                };
            }

            None
        })
        .for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });
}

fn is_sensor(layers: CollisionLayers) -> bool {
    layers.contains_group(GamePhysicsLayer::Sensor)
        && !layers.contains_group(GamePhysicsLayer::Player)
        && !layers.contains_group(GamePhysicsLayer::Projectile)
        && !layers.contains_group(GamePhysicsLayer::World)
        && !layers.contains_group(GamePhysicsLayer::Creature)
}
