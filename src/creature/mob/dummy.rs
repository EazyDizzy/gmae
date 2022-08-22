use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::creature::component::hp::HP;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use heron::Velocity;

pub fn insert(entity_commands: &mut EntityCommands) {
    entity_commands
        .insert(PhysiologyDescription::default())
        .insert(Velocity::default())
        .insert(HP::full(100));
        // .insert(HP::full(u16::MAX));
}
