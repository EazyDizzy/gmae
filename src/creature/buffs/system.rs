use bevy::prelude::*;
use crate::creature::buffs::{BuffStorage};
use crate::player::PlayerMarker;

pub fn apply_buffs<Target: Component>(mut query: Query<(&mut BuffStorage<Target>, &mut Target)>)  {
    for (mut buffs_component, mut target) in query.iter_mut() {
        buffs_component.apply(&mut target);
    }
}

pub fn clear_buffs<Target: Component>(mut query: Query<(&mut BuffStorage<Target>, &mut Target), With<PlayerMarker>>) {
    for (mut buffs_component, mut target) in query.iter_mut() {
        buffs_component.clean(&mut target);
    }
}