use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::creature::component::CombatParameters;
use crate::creature::EnemyCreatureMarker;
use crate::entity::component::hp::HP;
use crate::player::entity::Player;
use crate::util::round;
use crate::{entity, is_sensor, GamePhysicsLayer};
use bevy::prelude::*;
use heron::{CollisionEvent, CollisionLayers, CollisionShape, RigidBody};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};
use std::ops::Deref;

#[derive(Component)]
pub struct ThrustAttackSensor;

pub fn player_attack_thrust(
    keyboard_input: Res<Input<KeyCode>>,
    player: Query<(Entity, &CombatParameters, &PhysiologyDescription), With<Player>>,
    mut commands: Commands,
) {
    if !keyboard_input.just_pressed(KeyCode::O) {
        return;
    }
    let (e, combat, phys) = if let Ok(t) = player.get_single() {
        t
    } else {
        return;
    };

    commands.entity(e).with_children(|parent| {
        parent
            .spawn_bundle((
                Transform::from_xyz(0., 0., combat.attack_length / 2. + phys.model_radius),
                GlobalTransform::identity(),
            ))
            .insert(RigidBody::Sensor)
            .insert(ThrustAttackSensor)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(0.25, 0.5, combat.attack_length / 2.),
                border_radius: None,
            })
            .insert(
                CollisionLayers::all_masks::<GamePhysicsLayer>()
                    .with_group(GamePhysicsLayer::Sensor),
            );
    });
}
pub fn player_attack_thrust_check_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    thrust_sensors: Query<Entity, With<ThrustAttackSensor>>,
    mut enemies: Query<&mut HP, With<EnemyCreatureMarker>>,
    player: Query<&CombatParameters, With<Player>>,
    mut commands: Commands,
) {
    let sensor = if let Ok(s) = thrust_sensors.get_single() {
        s
    } else {
        return;
    };
    let combat = if let Ok(t) = player.get_single() {
        t
    } else {
        return;
    };

    collision_events
        .iter()
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            let with_sensor = is_sensor(layers_1) || is_sensor(layers_2);

            if with_sensor {
                if entity_1 == sensor {
                    return Some(entity_2);
                } else if entity_2 == sensor {
                    return Some(entity_1);
                };
            }

            None
        })
        .for_each(|entity| {
            if let Ok(mut hp) = enemies.get_mut(entity) {
                hp.make_damage(combat.base_damage);
            }
        });

    commands.entity(sensor).despawn();
}
