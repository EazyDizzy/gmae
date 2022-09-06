use crate::audio::DamageSoundType;
use bevy::prelude::Entity;

pub struct DamageEvent {
    pub target: Entity,
    pub amount: u16,
    // for sound event to be produced if the damage hits the entity
    pub sound_type: DamageSoundType,
}
