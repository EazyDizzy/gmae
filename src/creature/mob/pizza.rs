use crate::creature::component::attack::component::Attack;
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::creature::component::hp::HP;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub fn insert(entity_commands: &mut EntityCommands) {
    entity_commands
        .insert(PhysiologyDescription::default())
        // .insert(Attack::new())
        .insert(HP::full(100));
}
