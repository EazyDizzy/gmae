use crate::creature::component::attack::shooting::bullet::Bullet;
use crate::creature::component::movement::locomotivity::Locomotivity;
use crate::creature::component::physiology_description::PhysiologyDescription;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::reflect::List;
use heron::rapier_plugin::PhysicsWorld;
use heron::{Acceleration, CollisionShape, RigidBody, Velocity};
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
                    Bullet::new(player_position - eyes_pos, 0.1),
                    Transform::from_translation(eyes_pos),
                    GlobalTransform::identity(),
                ))
                .insert(CollisionShape::Sphere { radius: 0.2 })
                .insert(Velocity::from_linear(Vec3::default()))
                .insert(Acceleration::from_linear(Vec3::default()))
                .insert(RigidBody::Sensor);
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

fn speed_from_vector(vector_diff: f32, speed: f32) -> f32 {
    let mut min = if vector_diff.abs() < speed {
        vector_diff.abs()
    } else {
        speed
    };

    if vector_diff < 0.0 {
        min = -min;
    }

    min
}
