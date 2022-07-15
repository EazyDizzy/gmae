use crate::creature::component::movement::locomotivity::Locomotivity;
use crate::creature::component::movement::random_movement::RandomMovementStrategy;
use crate::creature::component::physiology_description::PhysiologyDescription;
use bevy::prelude::*;
use heron::Velocity;
use lib::entity::level::Level;
use std::fmt::Debug;

pub mod locomotivity;
pub mod random_movement;

pub trait MoveYourBody: Send + Sync + Debug {
    fn update(
        &mut self,
        locomotivity: &mut Locomotivity,
        phys: &PhysiologyDescription,
        lvl: &Res<Level>,
        transform: &Transform,
        velocity: &mut Velocity,
    );
}

#[derive(Component, Debug)]
pub struct MovementStrategy {
    pub strategy: Box<dyn MoveYourBody>,
}

impl MovementStrategy {
    pub fn random() -> MovementStrategy {
        MovementStrategy {
            strategy: Box::new(RandomMovementStrategy::new()),
        }
    }

    pub fn update(
        &mut self,
        locomotivity: &mut Locomotivity,
        phys: &PhysiologyDescription,
        lvl: &Res<Level>,
        transform: &Transform,
        velocity: &mut Velocity,
    ) {
        self.strategy
            .update(locomotivity, phys, lvl, transform, velocity);
    }
}
