use crate::creature::component::attack::util::raytracer::can_see_from;
use crate::creature::component::physiology_description::PhysiologyDescription;
use bevy::prelude::*;
use lib::entity::level::Level;
use lib::entity::point::Point;
use crate::creature::component::movement::locomotivity::Locomotivity;

pub mod shooting;
pub mod util;

#[derive(Component, Debug)]
pub struct Attack {}

impl Attack {
    pub fn exec(
        &self,
        phys: &PhysiologyDescription,
        locomotivity: &Locomotivity,
        transform: &Transform,
        player_position: &Point,
        lvl: &Level,
    ) {
        let pos: &Point = locomotivity.position();
        let eyes_pos = phys.get_eyes_position(transform, pos);
        if can_see_from(&eyes_pos, player_position, lvl) {
            // dbg!("can see");
        } else {
            // dbg!("can't see");
        }
    }
}
