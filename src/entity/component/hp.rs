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
    pub fn percent(&self) -> f32 {
        self.current as f32 / self.max as f32
    }

    pub fn max(&self) -> u16 {
        self.max
    }
    pub fn current(&self) -> u16 {
        self.current
    }
}
