use crate::audio::{DamageSoundType, SoundEvent, SoundLayer, SoundType};
use crate::creature::component::attack::event::DamageEvent;
use crate::creature::component::attack::number;
use crate::creature::component::attack::number::DamageNumberAssets;
use crate::creature::component::attack::shooting::bullet::Bullet;
use crate::creature::component::hp::HP;
use crate::particle::PunchEffect;
use crate::GamePhysicsLayer;
use bevy::prelude::*;
use bevy_hanabi::ParticleEffect;
use heron::{CollisionEvent, CollisionLayers};
use rand::Rng;
use std::cmp;
use std::f32::consts::{FRAC_PI_6, PI};

pub fn attack_despawn_killed_entities(mut commands: Commands, entities: Query<(Entity, &HP)>) {
    for (entity, hp) in entities.iter() {
        if hp.is_empty() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn attack_launch_bullets(mut bullets: Query<(&mut Transform, &Bullet)>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        transform.translation += bullet.shift;
    }
}

pub fn attack_apply_damage(
    mut commands: Commands,
    mut damage_events: EventReader<DamageEvent>,
    mut sound_events: EventWriter<SoundEvent>,
    mut entities: Query<(&mut HP, &Transform)>,
    numbers: Res<DamageNumberAssets>,
    mut hit_effects: Query<(&mut ParticleEffect, &mut Transform), (With<PunchEffect>, Without<HP>)>,
) {
    let (mut effect, mut effect_transform) = hit_effects.single_mut();

    for ev in damage_events.iter() {
        if let Ok((mut hp, transform)) = entities.get_mut(ev.target) {
            hp.sub(ev.amount);
            sound_events.send(SoundEvent {
                sound_layer: SoundLayer::ForeGround,
                sound_type: SoundType::Damage(ev.sound_type),
            });

            number::spawn(&mut commands, &numbers, &transform, ev.amount);

            effect_transform.translation = transform.translation;
            effect_transform.rotation =
                Quat::from_euler(EulerRot::XYZ, -FRAC_PI_6, -(PI - FRAC_PI_6), 0.);
            effect.maybe_spawner().unwrap().reset();
        }
    }
}

pub fn attack_check_bullet_collisions(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut ev_damage: EventWriter<DamageEvent>,
    bullets: Query<&Bullet>,
) {
    let mut rng = rand::thread_rng();

    events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            let with_bullet = is_bullet(layers_1) || is_bullet(layers_2);
            let with_player = is_player(layers_1) || is_player(layers_2);

            if with_bullet {
                if is_bullet(layers_1) {
                    commands.entity(entity_1).despawn();
                } else {
                    commands.entity(entity_2).despawn();
                }
            }

            if with_bullet && with_player {
                if is_player(layers_1) {
                    return Some((entity_1, entity_2));
                }
                return Some((entity_2, entity_1));
            }

            None
        })
        .for_each(|(target, bullet)| {
            let base_damage = bullets.get(bullet).expect("Bullet should exist").damage;
            let damage = rng.gen_range(
                cmp::min(base_damage - (base_damage / 10), base_damage - 1)
                    ..cmp::max(base_damage + (base_damage / 10), base_damage + 1),
            );
            ev_damage.send(DamageEvent {
                target,
                amount: damage,
                sound_type: DamageSoundType::Bullet,
            });
        });
}

fn is_bullet(layers: CollisionLayers) -> bool {
    layers.contains_group(GamePhysicsLayer::Projectile)
        && !layers.contains_group(GamePhysicsLayer::Player)
        && !layers.contains_group(GamePhysicsLayer::World)
        && !layers.contains_group(GamePhysicsLayer::Creature)
}
fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(GamePhysicsLayer::Player)
        && !layers.contains_group(GamePhysicsLayer::Projectile)
        && !layers.contains_group(GamePhysicsLayer::World)
        && !layers.contains_group(GamePhysicsLayer::Creature)
}
