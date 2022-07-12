use crate::creature::component::movement::locomotivity::Locomotivity;
use crate::creature::component::physiology_description::PhysiologyDescription;
use bevy::prelude::*;
use heron::rapier_plugin::PhysicsWorld;
use lib::entity::level::Level;
use lib::entity::point::Point;
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
    ) {
        // if self.time_of_last_attack.elapsed().as_secs() < 3 {
        //     return;
        // }
        let eyes_pos = phys.get_eyes_position(transform);
        let ray_cast_result = physics_world
            .ray_cast(eyes_pos.into_vec3(), player_position, true);

        if let Some(cast_info) = ray_cast_result
        {
            dbg!(cast_info.collision_point);
            dbg!("can't see");
        } else {
            self.time_of_last_attack = Instant::now();
            dbg!("can see");
        }
    }
}
