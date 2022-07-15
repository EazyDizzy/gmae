use crate::creature::component::attack::shooting::bullet::Bullet;
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::GamePhysicsLayer;
use bevy::prelude::*;
use heron::rapier_plugin::PhysicsWorld;
use heron::{Acceleration, CollisionLayers, CollisionShape, RigidBody, Velocity};
use std::time::Instant;

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
        mut commands: &mut Commands,
        mut meshes: &mut ResMut<Assets<Mesh>>,
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
            spawn_bullet(&mut commands, &mut meshes, player_position, eyes_pos);
        }
    }
}

fn spawn_bullet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    player_position: Vec3,
    eyes_pos: Vec3,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.1,
                ..Default::default()
            })),
            ..Default::default()
        })
        .insert_bundle((
            Bullet::new(player_position - eyes_pos, 0.1, 55),
            Transform::from_translation(eyes_pos),
            GlobalTransform::identity(),
        ))
        .insert(CollisionShape::Sphere { radius: 0.1 })
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
