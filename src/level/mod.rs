use crate::{is_sensor, GamePhysicsLayer};
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
        .spawn_bundle(TransformBundle::from_transform(Transform::from_xyz(
            0., -5., 0.,
        )))
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
    killing_sensors: Query<&KillingSensor>,
) {
    collision_events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            let with_sensor = is_sensor(layers_1) || is_sensor(layers_2);

            if with_sensor {
                if let Ok(..) = killing_sensors.get(entity_1) {
                    return Some(entity_2);
                } else if let Ok(..) = killing_sensors.get(entity_2) {
                    return Some(entity_1);
                };
            }

            None
        })
        .for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });
}
