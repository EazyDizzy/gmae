use std::f32::consts::PI;

use bevy::math::vec3;
use bevy::prelude::*;
use lib::entity::level::Level;
use lib::entity::point::Point;
use once_cell::sync::Lazy;

use crate::Transform;
use crate::util::round_based;

#[derive(Copy, Clone, Debug)]
enum MovementState {
    Falling,
    Jumping(u8),
}

#[derive(Component, Debug)]
pub struct Player {
    position: Point,
    movement_state: Option<MovementState>,
}

const MOVEMENT_SPEED: f32 = 0.1;
const GRAVITY_SPEED: f32 = MOVEMENT_SPEED;
const MODEL_RADIUS: f32 = 0.5;
static DEFAULT_ROTATION: Lazy<Quat> = Lazy::new(|| {
    Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)
});

impl Player {
    pub fn new() -> Player {
        Player {
            position: Point::new(10.0, 11.5, 3.0),
            movement_state: None,
        }
    }

    pub fn position(&self) -> &Point {
        &self.position
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
    pub fn jump(&mut self, lvl: &Res<Level>) {
        if self.movement_state.is_none() && self.can_jump(lvl) {
            self.movement_state = Some(MovementState::Jumping(0));
        }
    }

    pub fn gravity_move(&mut self, lvl: &Res<Level>) {
        if self.movement_state.is_none() {
            if self.should_fall(lvl) {
                self.movement_state = Some(MovementState::Falling);
            } else {
                return;
            }
        }

        match self.movement_state.unwrap() {
            MovementState::Falling => {
                let z_gap = self.position.z - self.position.z.floor();
                if self.can_fall(lvl) {
                    self.position.z -= GRAVITY_SPEED;
                } else if z_gap > 0.0 {
                    let gravity_speed = if z_gap > GRAVITY_SPEED { GRAVITY_SPEED } else { z_gap };
                    self.position.z -= gravity_speed;
                }

                if !self.should_fall(lvl) {
                    self.movement_state = None;
                }
            }
            MovementState::Jumping(tick) => {
                self.position.z += GRAVITY_SPEED;

                if tick == 10 {
                    self.movement_state = None;
                } else {
                    self.movement_state = Some(MovementState::Jumping(tick + 1));
                }
            }
        }
    }

    pub fn move_model(&self, position: &mut Transform) {
        *position = Transform::from_xyz(self.position.x, self.position.y, self.position.z + 0.5)
            .with_scale(vec3(0.5, 0.5, 0.5))
            .with_rotation(*DEFAULT_ROTATION);
    }
}

// obstacles checks
impl Player {
    fn can_jump(&self, lvl: &Res<Level>) -> bool {
        self.has_fundament(lvl) && !self.has_ceil(lvl)
    }

    fn should_fall(&self, lvl: &Res<Level>) -> bool {
        self.position.z - self.position.z.floor() != 0.0 || !self.has_fundament(lvl)
    }
    fn can_fall(&self, lvl: &Res<Level>) -> bool {
        let future_z = self.position.z.floor();

        self.no_z_obstacles(future_z, lvl)
    }

    fn has_fundament(&self, lvl: &Res<Level>) -> bool {
        let mut points = vec![
            Point::new(self.position.x.floor(), self.position.y.floor(), self.position.z.floor()),
        ];

        let x_gap = round_based(self.position.x - self.position.x.floor(), 1);
        if x_gap > MODEL_RADIUS {
            points.push(Point::new(self.position.x.round(), self.position.y.round(), self.position.z.floor()));
        } else if x_gap < MODEL_RADIUS {
            points.push(Point::new((self.position.x - MODEL_RADIUS).floor(), self.position.y.round(), self.position.z.floor()));
        }

        let y_gap = round_based(self.position.y - self.position.y.floor(), 1);
        if y_gap > MODEL_RADIUS {
            points.push(Point::new(self.position.x.round(), self.position.y.round(), self.position.z.floor()));
        } else if y_gap < MODEL_RADIUS {
            points.push(Point::new(self.position.x.round(), (self.position.y - MODEL_RADIUS).floor(), self.position.z.floor()));
        }

        !self.all_air(&points, lvl)
    }
    fn has_ceil(&self, lvl: &Res<Level>) -> bool {
        let future_position = Point::new(self.position.x.round(), self.position.y.round(), (self.position.z + 3.0).floor());
        let voxel_ceil = lvl.get_voxel_by_point(&future_position);

        voxel_ceil.is_some()
    }

    fn no_z_obstacles(&self, z: f32, lvl: &Res<Level>) -> bool {
        const FALLING_MODEL_RADIUS: f32 = MODEL_RADIUS * 0.9;
        let obstacles = [
            Point::new((self.position.x + FALLING_MODEL_RADIUS).floor(), (self.position.y + FALLING_MODEL_RADIUS).floor(), z),
            Point::new((self.position.x + FALLING_MODEL_RADIUS).floor(), (self.position.y - FALLING_MODEL_RADIUS).floor(), z),
            Point::new((self.position.x - FALLING_MODEL_RADIUS).floor(), (self.position.y + FALLING_MODEL_RADIUS).floor(), z),
            Point::new((self.position.x - FALLING_MODEL_RADIUS).floor(), (self.position.y - FALLING_MODEL_RADIUS).floor(), z),
        ];

        self.all_air(&obstacles, lvl)
    }
    fn no_x_obstacles(&self, y: f32, lvl: &Res<Level>) -> bool {
        let x_gap = round_based(self.position.x - self.position.x.floor(), 2);

        let obstacles = if x_gap == MODEL_RADIUS {
            vec![
                Point::new(self.position.x.floor(), y, self.position.z + 1.0),
                Point::new(self.position.x.floor(), y, self.position.z + 2.0),
            ]
        } else if x_gap > MODEL_RADIUS {
            vec![
                Point::new((self.position.x + MODEL_RADIUS).round(), y, self.position.z + 1.0),
                Point::new((self.position.x + MODEL_RADIUS).round(), y, self.position.z + 2.0),
                Point::new(self.position.x.floor(), y, self.position.z + 1.0),
                Point::new(self.position.x.floor(), y, self.position.z + 2.0),
            ]
        } else {
            vec![
                Point::new((self.position.x - MODEL_RADIUS).floor(), y, self.position.z + 1.0),
                Point::new((self.position.x - MODEL_RADIUS).floor(), y, self.position.z + 2.0),
                Point::new(self.position.x.floor(), y, self.position.z + 1.0),
                Point::new(self.position.x.floor(), y, self.position.z + 2.0),
            ]
        };

        self.all_air(&obstacles, lvl)
    }
    fn no_y_obstacles(&self, x: f32, lvl: &Res<Level>) -> bool {
        let y_gap = round_based(self.position.y - self.position.y.floor(), 2);

        let obstacles = if y_gap == MODEL_RADIUS {
            vec![
                Point::new(x, self.position.y.floor(), self.position.z + 1.0),
                Point::new(x, self.position.y.floor(), self.position.z + 2.0),
            ]
        } else if y_gap > MODEL_RADIUS {
            vec![
                Point::new(x, (self.position.y + MODEL_RADIUS).round(), self.position.z + 1.0),
                Point::new(x, (self.position.y + MODEL_RADIUS).round(), self.position.z + 2.0),
                Point::new(x, self.position.y.floor(), self.position.z + 1.0),
                Point::new(x, self.position.y.floor(), self.position.z + 2.0),
            ]
        } else {
            vec![
                Point::new(x, (self.position.y - MODEL_RADIUS).floor(), self.position.z + 1.0),
                Point::new(x, (self.position.y - MODEL_RADIUS).floor(), self.position.z + 2.0),
                Point::new(x, self.position.y.floor(), self.position.z + 1.0),
                Point::new(x, self.position.y.floor(), self.position.z + 2.0),
            ]
        };

        self.all_air(&obstacles, lvl)
    }

    fn all_air(&self, points: &[Point], lvl: &Res<Level>) -> bool {
        points.iter()
            .all(|p| lvl.get_voxel_by_point(p).is_none())
    }
}

