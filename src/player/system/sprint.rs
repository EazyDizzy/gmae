use bevy::prelude::*;
use crate::creature::buffs::{BuffClock, BuffStorage, PhysiologyBuff, BuffTimer, SprintBuff};
use crate::player::entity::Player;

pub fn add_sprint(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut BuffStorage, With<Player>>) {
    let mut buffs_component = query.single_mut();

    keyboard_input.get_pressed().for_each(|k| {
        match k {
            KeyCode::LShift | KeyCode::RShift => {
                buffs_component.physiology_buffs.push(BuffClock::frame(Box::new(SprintBuff::new()), 1));
            },
            _ => {},
        };

    });
}
