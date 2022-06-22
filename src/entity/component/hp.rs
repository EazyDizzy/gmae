use bevy::prelude::*;

#[derive(Component)]
pub struct HP {
    max: u16,
    current: u16,
}

impl HP {
    pub fn full(max: u16) -> HP {
        HP { max, current: max }
    }
}
