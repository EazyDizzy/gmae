use crate::creature::component::movement::locomotivity;
use crate::creature::component::movement::MoveYourBody;
use crate::creature::component::physiology_description::PhysiologyDescription;
use bevy::math::vec3;
use bevy::prelude::*;
use heron::Velocity;
use lib::entity::level::Level;
use rand::Rng;

#[derive(Debug)]
pub struct RandomMovementStrategy {
    i: u8,
    direction: usize,
}

impl RandomMovementStrategy {
    pub fn new() -> Self {
        RandomMovementStrategy { i: 0, direction: 0 }
    }
}

impl MoveYourBody for RandomMovementStrategy {
    fn update(
        &mut self,
        phys: &PhysiologyDescription,
        lvl: &Res<Level>,
        transform: &Transform,
        velocity: &mut Velocity,
    ) {
        let x = transform.translation.x;
        let y = transform.translation.y;
        let z = transform.translation.z;
        let distance = 0.5;
        let potential_positions = [
            (x, z),
            (x + distance, z),
            (x, z + distance),
            (x + distance, z + distance),
            (x - distance, z),
            (x, z - distance),
            (x - distance, z - distance),
            (x - distance, z + distance),
            (x + distance, z - distance),
        ];
        let valid_positions: Vec<(f32, f32)> = potential_positions
            .into_iter()
            .filter(|(x, z)| {
                locomotivity::creature_not_inside_blocks(*x, y, *z, lvl, phys)
                    // && locomotivity::has_y_obstacles_on_point(*x, y, *z, lvl, phys)
            })
            .collect();

        if valid_positions.is_empty() {
            return;
        }

        let index = if self.i < 20 && valid_positions.get(self.direction).is_some() {
            self.direction
        } else {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..valid_positions.len());
            self.i = 0;
            self.direction = index;
            index
        };
        // dbg!(potential_positions);

        let (new_x, new_z) = potential_positions[index];
        let diff_x = transform.translation.x - new_x;
        let diff_z = transform.translation.z - new_z;
        *velocity = velocity.with_linear(vec3(
            diff_x * phys.movement_speed,
            velocity.linear.y,
            diff_z * phys.movement_speed,
        ));

        self.i += 1;
    }
}
