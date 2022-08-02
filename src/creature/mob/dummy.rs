use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::entity::component::hp::HP;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use heron::Velocity;

#[derive(Component, Debug)]
pub struct Dummy {}

impl Dummy {
    pub fn new() -> Self {
        Dummy {}
    }
}

pub fn insert(entity_commands: &mut EntityCommands) {
    entity_commands
        .insert(PhysiologyDescription::default())
        .insert(Velocity::default())
        .insert(HP::full(100));
}
