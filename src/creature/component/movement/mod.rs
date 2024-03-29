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
    #[allow(dead_code)]
    pub fn random() -> MovementStrategy {
        MovementStrategy {
            strategy: Box::new(RandomMovementStrategy::new()),
        }
    }

    pub fn update(
        &mut self,
        phys: &PhysiologyDescription,
        lvl: &Res<Level>,
        transform: &Transform,
        velocity: &mut Velocity,
    ) {
        self.strategy.update(phys, lvl, transform, velocity);
    }
}
