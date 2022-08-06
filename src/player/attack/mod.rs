use crate::audio::DamageSoundType;
use crate::creature::component::attack::event::DamageEvent;
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::creature::component::CombatParameters;
use crate::player::entity::Player;
use crate::{is_sensor, GamePhysicsLayer};
use bevy::prelude::*;
use heron::{CollisionEvent, CollisionLayers, CollisionShape, RigidBody};
use rand::Rng;

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
    player: Query<&CombatParameters, With<Player>>,
    mut commands: Commands,
    mut ev_damage: EventWriter<DamageEvent>,
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
    let mut rng = rand::thread_rng();

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
        .for_each(|target| {
            let damage = rng.gen_range(
                combat.base_damage - (combat.base_damage / 10)
                    ..=combat.base_damage + (combat.base_damage / 10),
            );

            ev_damage.send(DamageEvent {
                target,
                amount: damage,
                sound_type: DamageSoundType::Punch,
            });
        });

    commands.entity(sensor).despawn();
}
