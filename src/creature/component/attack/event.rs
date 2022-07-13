use bevy::prelude::Entity;

pub struct DamageEvent {
    pub target: Entity,
    pub amount: u16,
}
