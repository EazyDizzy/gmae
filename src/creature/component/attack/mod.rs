use crate::creature::component::attack::event::DamageEvent;
use crate::creature::component::attack::system::{
    apply_damage, launch_bullets, make_damage_from_bullet,
};
use crate::GameState;
use bevy::prelude::*;

pub mod component;
pub mod event;
pub mod shooting;
mod system;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(launch_bullets)
                .with_system(apply_damage)
                .with_system(make_damage_from_bullet),
        );
    }
}
