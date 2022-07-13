use bevy::prelude::*;
use crate::creature::buffs::{BuffStorage};
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::player::entity::Player;

pub fn apply_buffs(mut query: Query<(&mut BuffStorage, &mut PhysiologyDescription), With<Player>>) {
    for (mut buffs_component, mut phys) in query.iter_mut() {
        buffs_component.apply(&mut phys);
    }
}

pub fn clear_buffs(mut query: Query<(&mut BuffStorage, &mut PhysiologyDescription), With<Player>>) {
    for (mut buffs_component, mut phys) in query.iter_mut() {
        buffs_component.clean(&mut phys);
    }
}