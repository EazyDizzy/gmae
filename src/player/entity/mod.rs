use std::f32::consts::PI;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use lib::entity::level::Level;
use lib::entity::point::Point;
use once_cell::sync::Lazy;

use crate::{Entity, Query, Transform};

pub struct Player {
    id: Entity,
    position: Point,
}

const MOVEMENT_SPEED: f32 = 0.1;
static DEFAULT_ROTATION: Lazy<Quat> = Lazy::new(|| {
    Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)
});

impl Player {
    pub fn new(id: Entity) -> Player {
        Player {
            id,
            position: Point::new(3.0, 3.0, 0.0),
        }
    }

    pub fn move_back(&mut self, x: &Res<Level>) {
        self.position.y -= MOVEMENT_SPEED;
    }
    pub fn move_forward(&mut self, lvl: &Res<Level>) {
        let future_y = self.position.y + MOVEMENT_SPEED;

        self.position.y += MOVEMENT_SPEED;
    }
    pub fn move_left(&mut self, x: &Res<Level>) {
        self.position.x -= MOVEMENT_SPEED;
    }
    pub fn move_right(&mut self, x: &Res<Level>) {
        self.position.x += MOVEMENT_SPEED;
    }

    pub fn move_model(&self, mut transforms: Query<&mut Transform>) {
        let mut position = transforms.get_mut(self.id).unwrap();
        *position = Transform::from_xyz(self.position.x, self.position.y, self.position.z - 0.5)
            .with_rotation(*DEFAULT_ROTATION);
    }
}