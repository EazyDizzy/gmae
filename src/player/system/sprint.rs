use std::time::Duration;
use bevy::prelude::*;
use crate::creature::buffs::{BuffClock, BuffStorage, PhysiologyBuff, BuffTimer, SprintBuff};
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::player::entity::Player;

pub fn buffs_add_sprint(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut BuffStorage, With<Player>>) {
    for mut buffs in query.iter_mut() {
        keyboard_input.get_pressed().for_each(|k| {
            match k {
                KeyCode::LShift | KeyCode::RShift => {
                    buffs.physiology_buffs.push(BuffClock::frame(Box::new(SprintBuff::default()), 1));
                },
                _ => {},
            };

        });
    }
}

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