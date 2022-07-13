use crate::creature::component::attack::shooting::bullet::Bullet;
use crate::creature::component::movement::locomotivity::Locomotivity;
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::creature::event::DamageEvent;
use crate::entity::component::hp::HP;
use crate::GamePhysicsLayer;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::reflect::List;
use heron::rapier_plugin::PhysicsWorld;
use heron::{Acceleration, CollisionEvent, CollisionLayers, CollisionShape, RigidBody, Velocity};
use lib::entity::level::Level;
use lib::entity::point::Point;
use std::cmp;
use std::time::Instant;

pub mod shooting;

#[derive(Component, Debug)]
pub struct Attack {
    time_of_last_attack: Instant,
}

impl Attack {
    pub fn new() -> Attack {
        Attack {
            time_of_last_attack: Instant::now(),
        }
    }

    pub fn exec(
        &mut self,
        physics_world: &PhysicsWorld,
        phys: &PhysiologyDescription,
        transform: &Transform,
        player_position: Vec3,
        player_entity: Entity,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        if self.time_of_last_attack.elapsed().as_secs() < 1 {
            return;
        }
        let eyes_pos = phys.get_eyes_position(transform);
        let can_see = physics_world
            .ray_cast(eyes_pos, player_position - eyes_pos, true)
            .map_or(true, |cast| cast.entity == player_entity);

        if can_see {
            self.time_of_last_attack = Instant::now();
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.2,
                        ..Default::default()
                    })),
                    ..Default::default()
                })
                .insert_bundle((
                    Bullet::new(player_position - eyes_pos, 0.1, 3),
                    Transform::from_translation(eyes_pos),
                    GlobalTransform::identity(),
                ))
                .insert(CollisionShape::Sphere { radius: 0.2 })
                .insert(Velocity::from_linear(Vec3::default()))
                .insert(Acceleration::from_linear(Vec3::default()))
                .insert(RigidBody::Sensor)
                .insert(
                    CollisionLayers::none()
                        .with_group(GamePhysicsLayer::Projectile)
                        .with_masks([
                            GamePhysicsLayer::Creature,
                            GamePhysicsLayer::Player,
                            GamePhysicsLayer::World,
                        ]),
                );
        }
    }
}

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
