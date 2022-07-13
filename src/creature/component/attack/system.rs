use bevy::prelude::*;
use heron::{CollisionEvent, CollisionLayers};
use crate::creature::component::attack::event::DamageEvent;
use crate::creature::component::attack::shooting::bullet::Bullet;
use crate::entity::component::hp::HP;
use crate::GamePhysicsLayer;

pub fn launch_bullets(mut bullets: Query<(&mut Transform, &Bullet)>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        transform.translation.x += bullet.shift.x;
        transform.translation.y += bullet.shift.y;
        transform.translation.z += bullet.shift.z;
    }
}

pub fn apply_damage(mut ev_damage: EventReader<DamageEvent>, mut hps: Query<&mut HP>) {
    for ev in ev_damage.iter() {
        if let Ok(mut hp) = hps.get_mut(ev.target) {
            hp.sub(ev.amount);
        }
    }
}

pub fn make_damage_from_bullet(
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
            if (is_bullet(layers_1) || is_bullet(layers_2))
                && (is_player(layers_1) || is_player(layers_2))
            {
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
            commands.entity(bullet).despawn();
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