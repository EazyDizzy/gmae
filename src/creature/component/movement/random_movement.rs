use crate::creature::component::movement::locomotivity::Locomotivity;
use crate::creature::component::movement::MoveYourBody;
use crate::creature::component::physiology_description::PhysiologyDescription;
use bevy::prelude::*;
use lib::entity::level::Level;
use rand::Rng;

#[derive(Debug)]
pub struct RandomMovementStrategy {
    i: u8,
    direction: usize,
}

impl RandomMovementStrategy {
    pub fn new() -> RandomMovementStrategy {
        RandomMovementStrategy { i: 0, direction: 0 }
    }
}

impl MoveYourBody for RandomMovementStrategy {
    fn update(
        &mut self,
        locomotivity: &mut Locomotivity,
        phys: &PhysiologyDescription,
        lvl: &Res<Level>,
    ) {
        let position = locomotivity.position();
        let x = position.x;
        let z = position.z;
        let speed = phys.movement_speed;
        let potential_positions = [
            (x, z),
            (x + speed, z),
            (x, z + speed),
            (x + speed, z + speed),
            (x - speed, z),
            (x, z - speed),
            (x - speed, z - speed),
            (x - speed, z + speed),
            (x + speed, z - speed),
        ];
        let valid_positions: Vec<(f32, f32)> = potential_positions
            .into_iter()
            .filter(|(x, z)| locomotivity.can_stay_on(*x, *z, lvl, phys))
            .collect();

        let index = if self.i < 20 && valid_positions.get(self.direction).is_some() {
            self.direction
        } else {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..valid_positions.len());
            self.i = 0;
            self.direction = index;
            index
        };

        let (new_x, new_z) = potential_positions[index];
        locomotivity.move_to(new_x, new_z, lvl, phys);

        self.i += 1;
    }
}
