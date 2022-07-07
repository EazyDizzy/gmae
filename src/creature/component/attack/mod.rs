use crate::creature::component::attack::util::raytracer::can_see_from;
use crate::creature::component::movement::locomotivity::Locomotivity;
use crate::creature::component::physiology_description::PhysiologyDescription;
use bevy::prelude::*;
use lib::entity::level::Level;
use lib::entity::point::Point;
use std::time::Instant;

pub mod shooting;
pub mod util;

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
        phys: &PhysiologyDescription,
        locomotivity: &Locomotivity,
        transform: &Transform,
        player_position: &Point,
        lvl: &Level,
    ) {
        if self.time_of_last_attack.elapsed().as_secs() < 3 {
            return;
        }
        let pos: &Point = locomotivity.position();
        let eyes_pos = phys.get_eyes_position(transform, pos);
        if can_see_from(&eyes_pos, player_position, lvl) {

            self.time_of_last_attack = Instant::now();
        }
    }
}
