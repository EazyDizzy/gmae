use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct CombatParameters {
    pub attack_length: f32,
}

impl Default for CombatParameters {
    fn default() -> Self {
        CombatParameters { attack_length: 1.5 }
    }
}
