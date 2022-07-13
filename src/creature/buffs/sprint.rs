use bevy::prelude::*;
use crate::creature::buffs::{Buff, BuffClock, BuffStorage};
use crate::creature::component::physiology_description::PhysiologyDescription;
use crate::player::entity::Player;

#[derive(Debug)]
pub struct SprintBuff {
    speed_multiplier: f32
}

impl Default for SprintBuff {
    fn default() -> Self {
        SprintBuff {
            speed_multiplier: 1.5
        }
    }
}

impl Buff<PhysiologyDescription> for SprintBuff {
    fn apply(&self, phys: &mut PhysiologyDescription) {
        phys.movement_speed *= self.speed_multiplier;
    }
    fn remove(&self, phys: &mut PhysiologyDescription) {
        phys.movement_speed = 0.1;
    }
}

pub fn buffs_add_sprint(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut BuffStorage<PhysiologyDescription>, With<Player>>) {
    for mut buffs_component in query.iter_mut() {
        keyboard_input.get_pressed().for_each(|k| {
            match k {
                KeyCode::LShift | KeyCode::RShift => {
                    buffs_component.buffs.push(BuffClock::frame(Box::new(SprintBuff::default()), 1));
                },
                _ => {},
            };

        });
    }
}