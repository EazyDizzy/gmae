use crate::creature::component::attack::event::DamageEvent;
use crate::creature::component::attack::shooting::bullet::Bullet;
use crate::entity::component::hp::HP;
use crate::GamePhysicsLayer;
use bevy::prelude::*;
use heron::{CollisionEvent, CollisionLayers};

pub fn attack_despawn_killed_entities(mut commands: Commands, entities: Query<(Entity, &HP)>) {
    for (entity, hp) in entities.iter() {
        if hp.is_empty() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn attack_launch_bullets(mut bullets: Query<(&mut Transform, &Bullet)>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        transform.translation.x += bullet.shift.x;
        transform.translation.y += bullet.shift.y;
        transform.translation.z += bullet.shift.z;
    }
}

pub fn attack_apply_damage(mut ev_damage: EventReader<DamageEvent>, mut hps: Query<&mut HP>) {
    for ev in ev_damage.iter() {
        if let Ok(mut hp) = hps.get_mut(ev.target) {
            hp.sub(ev.amount);
        }
    }
}

pub fn attack_check_bullet_collisions(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut ev_damage: EventWriter<DamageEvent>,
    bullets: Query<&Bullet>,
) {
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
            let damage = bullets.get(bullet).expect("Bullet should exist").damage;
            ev_damage.send(DamageEvent {
                target,
                amount: damage,
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
