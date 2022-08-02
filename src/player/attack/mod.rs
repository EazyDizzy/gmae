use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::creature::component::CombatParameters;
use crate::player::entity::Player;
use crate::GamePhysicsLayer;
use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

#[derive(Component)]
pub struct ThrustAttackSensor;

pub fn player_attack_setup_sensors(
    mut commands: Commands,
    player: Query<(&Transform, &CombatParameters), With<Player>>,
) {
    let (player_transform, comb) = if let Ok(t) = player.get_single() {
        t
    } else {
        return;
    };

    commands
        .spawn_bundle((player_transform.clone(), GlobalTransform::identity()))
        .insert(RigidBody::Sensor)
        .insert(ThrustAttackSensor)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(0.25, 0.5, comb.attack_length / 2.),
            border_radius: None,
        })
        .insert(
            CollisionLayers::all_masks::<GamePhysicsLayer>().with_group(GamePhysicsLayer::Sensor),
        );
}

pub fn player_attack_move_sensor(
    player: Query<
        (&Transform, &PhysiologyDescription),
        (With<Player>, Without<ThrustAttackSensor>),
    >,
    mut sensors: Query<&mut Transform, (With<ThrustAttackSensor>, Without<Player>)>,
) {
    let (player_transform, phys) = if let Ok(t) = player.get_single() {
        t
    } else {
        return;
    };
    let (_, mut y, _) = player_transform.rotation.to_euler(EulerRot::XYZ);
    y = round(y, 2);
    let rotation: Quat = player_transform.rotation;

    if rotation.y.abs() > rotation.w.abs() {
        if y < 0. {
            y -= FRAC_PI_2;
        } else if y > 0. {
            y += FRAC_PI_2;
        } else {
            y = PI;
        }
    }

    let distance = phys.model_radius * 2.5;
    let x: f32 = distance * y.sin() + player_transform.translation.x;
    let z: f32 = distance * y.cos() + player_transform.translation.z;

    for mut sensor in sensors.iter_mut() {
        *sensor = Transform::from_xyz(x, player_transform.translation.y, z)
            .with_rotation(player_transform.rotation);
    }
}

pub fn player_attack_thrust(keyboard_input: Res<Input<KeyCode>>) {
    if !keyboard_input.just_pressed(KeyCode::O) {
        return;
    }
}

fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}
