use bevy::prelude::*;
use std::time::{Duration, Instant};

#[derive(Component)]
pub struct ShortLife {
    born_at: Instant,
    lifetime: Duration,
}

impl ShortLife {
    pub fn new(lifetime: Duration) -> Self {
        ShortLife {
            born_at: Instant::now(),
            lifetime,
        }
    }

    pub fn outdated(&self) -> bool {
        self.born_at.elapsed() >= self.lifetime
    }
}

pub fn despawn_outdated_entities(mut numbers: Query<(&ShortLife, Entity)>, mut commands: Commands) {
    for (life, e) in numbers.iter_mut() {
        if life.outdated() {
            commands.entity(e).despawn_recursive();
        }
    }
}
