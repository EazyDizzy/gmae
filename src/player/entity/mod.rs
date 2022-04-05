use std::f32::consts::PI;

use bevy::math::vec3;
use bevy::prelude::*;
use lib::entity::level::Level;
use lib::entity::point::Point;
use once_cell::sync::Lazy;

use crate::{Entity, Query, Transform};
use crate::util::round_based;

pub struct Player {
    id: Entity,
    position: Point,
}

const MOVEMENT_SPEED: f32 = 0.1;
const GRAVITY_SPEED: f32 = MOVEMENT_SPEED;
const MODEL_RADIUS: f32 = 0.5;
static DEFAULT_ROTATION: Lazy<Quat> = Lazy::new(|| {
    Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)
});

impl Player {
    pub fn new(id: Entity) -> Player {
        Player {
            id,
            position: Point::new(10.0, 11.5, 3.0),
        }
    }

    pub fn move_back(&mut self, lvl: &Res<Level>) {
        let future_y = (self.position.y - MOVEMENT_SPEED / 2.0 - MODEL_RADIUS).floor();

        if self.no_x_obstacles(future_y, lvl) {
            self.position.y -= MOVEMENT_SPEED;
        }
    }
    pub fn move_forward(&mut self, lvl: &Res<Level>) {
        let future_y = (self.position.y + MOVEMENT_SPEED / 2.0 + MODEL_RADIUS).floor();

        if self.no_x_obstacles(future_y, lvl) {
            self.position.y += MOVEMENT_SPEED;
        }
    }
    pub fn move_left(&mut self, lvl: &Res<Level>) {
        let future_x = (self.position.x - MOVEMENT_SPEED / 2.0 - MODEL_RADIUS).floor();

        if self.no_y_obstacles(future_x, lvl) {
            self.position.x -= MOVEMENT_SPEED;
        }
    }
    pub fn move_right(&mut self, lvl: &Res<Level>) {
        let future_x = (self.position.x + MOVEMENT_SPEED / 2.0 + MODEL_RADIUS).floor();

        if self.no_y_obstacles(future_x, lvl) {
            self.position.x += MOVEMENT_SPEED;
        }
    }

    pub fn gravity_move(&mut self, lvl: &Res<Level>) {
        let z_gap = self.position.z - self.position.z.floor();
        let current_position = Point::new(self.position.x.round(), self.position.y.round(), self.position.z.floor());
        let voxel_fundament = lvl.get_voxel_by_point(current_position);

        if voxel_fundament.is_none() {
            self.position.z -= GRAVITY_SPEED;
        } else if z_gap > 0.0 {
            let gravity_speed = if z_gap > GRAVITY_SPEED { GRAVITY_SPEED } else { z_gap };
            self.position.z -= gravity_speed;
        }
    }

    pub fn move_model(&self, mut transforms: Query<&mut Transform>) {
        let mut position = transforms.get_mut(self.id).unwrap();
        *position = Transform::from_xyz(self.position.x, self.position.y, self.position.z + 0.5)
            .with_scale(vec3(0.5, 0.5, 0.5))
            .with_rotation(*DEFAULT_ROTATION);
    }
}

// obstacles checks
impl Player {
    fn no_x_obstacles(&self, future_y: f32, lvl: &Res<Level>) -> bool {
        let x_gap = round_based(self.position.x - self.position.x.floor(), 2);

        let obstacles = if x_gap > MODEL_RADIUS {
            [
                Point::new((self.position.x + MODEL_RADIUS).round(), future_y, self.position.z + 1.0),
                Point::new((self.position.x + MODEL_RADIUS).round(), future_y, self.position.z + 2.0),
                Point::new(self.position.x.floor(), future_y, self.position.z + 1.0),
                Point::new(self.position.x.floor(), future_y, self.position.z + 2.0),
            ]
        } else {
            [
                Point::new((self.position.x - MODEL_RADIUS).floor(), future_y, self.position.z + 1.0),
                Point::new((self.position.x - MODEL_RADIUS).floor(), future_y, self.position.z + 2.0),
                Point::new(self.position.x.floor(), future_y, self.position.z + 1.0),
                Point::new(self.position.x.floor(), future_y, self.position.z + 2.0),
            ]
        };

        self.all_air(obstacles, lvl)
    }

    fn no_y_obstacles(&self, future_x: f32, lvl: &Res<Level>) -> bool {
        let y_gap = round_based(self.position.y - self.position.y.floor(), 2);

        let obstacles = if y_gap > MODEL_RADIUS {
            [
                Point::new(future_x, (self.position.y + MODEL_RADIUS).round(), self.position.z + 1.0),
                Point::new(future_x, (self.position.y + MODEL_RADIUS).round(), self.position.z + 2.0),
                Point::new(future_x, self.position.y.floor(), self.position.z + 1.0),
                Point::new(future_x, self.position.y.floor(), self.position.z + 2.0),
            ]
        } else {
            [
                Point::new(future_x, (self.position.y - MODEL_RADIUS).floor(), self.position.z + 1.0),
                Point::new(future_x, (self.position.y - MODEL_RADIUS).floor(), self.position.z + 2.0),
                Point::new(future_x, self.position.y.floor(), self.position.z + 1.0),
                Point::new(future_x, self.position.y.floor(), self.position.z + 2.0),
            ]
        };

        self.all_air(obstacles, lvl)
    }

    fn all_air(&self, points: [Point; 4], lvl: &Res<Level>) -> bool {
        points.into_iter()
            .all(|p| lvl.get_voxel_by_point(p).is_none())
    }
}

