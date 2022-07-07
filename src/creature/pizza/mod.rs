use crate::creature::component::attack::Attack;
use crate::creature::component::movement::locomotivity::Locomotivity;
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::entity::component::hp::HP;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use lib::entity::level::creature::Creature;

#[derive(Component, Debug)]
pub struct Pizza {}

impl Pizza {
    pub fn new() -> Pizza {
        Pizza {}
    }
}

pub fn insert(entity_commands: &mut EntityCommands, creature: &Creature) {
    entity_commands
        .insert(PhysiologyDescription::default())
        .insert(Locomotivity::from(creature))
        .insert(Attack::new())
        .insert(HP::full(100));
}
