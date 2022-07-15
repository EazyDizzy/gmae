use crate::creature::component::movement::locomotivity::Locomotivity;
use crate::creature::component::movement::MovementStrategy;
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::entity::component::hp::HP;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use heron::Velocity;
use lib::entity::level::creature::Creature;

#[derive(Component, Debug)]
pub struct Dummy {}

impl Dummy {
    pub fn new() -> Dummy {
        Dummy {}
    }
}

pub fn insert(entity_commands: &mut EntityCommands, creature: &Creature) {
    entity_commands
        .insert(MovementStrategy::random())
        .insert(PhysiologyDescription::default())
        .insert(Locomotivity::new())
        .insert(Velocity::default())
        .insert(HP::full(100));
}
