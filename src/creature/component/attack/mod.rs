use crate::creature::component::attack::event::DamageEvent;
use crate::creature::component::attack::system::{
    attack_apply_damage, attack_check_bullet_collisions, attack_despawn_killed_entities,
    attack_launch_bullets,
};
use crate::GameState;
use bevy::prelude::*;
use crate::creature::component::attack::number::attack_animate_damage_numbers;

pub mod component;
pub mod event;
pub mod shooting;
mod system;
mod number;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(attack_despawn_killed_entities)
                .with_system(attack_launch_bullets)
                .with_system(attack_apply_damage)
                .with_system(attack_animate_damage_numbers)
                .with_system(attack_check_bullet_collisions),
        );
    }
}
