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
        f32::from(self.current) / f32::from(self.max)
    }

    pub fn max(&self) -> u16 {
        self.max
    }
    pub fn current(&self) -> u16 {
        self.current
    }
    pub fn sub(&mut self, amount: u16) {
        if amount < self.current {
            self.current -= amount;
        } else {
            self.current = 0;
        }
    }
}
