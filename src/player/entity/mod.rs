use std::f32::consts::{FRAC_PI_2, PI};

use bevy::math::vec3;
use bevy::prelude::*;
use lib::entity::level::Level;
use lib::entity::point::Point;
use lib::util::math::round_based;

use crate::player::entity::PiePiece::{BottomLeft, BottomRight, TopLeft, TopRight};
use crate::Transform;

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

#[derive(Copy, Clone, Debug)]
enum PiePiece {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Point::new(10.0, 3.0, 11.0),
            movement_state: None,
        }
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    pub fn move_forward(&mut self, lvl: &Res<Level>, angle: f32) {
        let (diff, piece) = if angle <= FRAC_PI_2 {
            (angle, PiePiece::BottomLeft)
        } else if angle <= PI {
            (angle - FRAC_PI_2, TopLeft)
        } else if angle <= PI + FRAC_PI_2 {
            (angle - PI, TopRight)
        } else {
            (angle - (PI + FRAC_PI_2), BottomRight)
        };

        let (x, z) = match piece {
            TopRight => {
                let z = diff / FRAC_PI_2;
                ((1.0 - z), z)
            }
            TopLeft => {
                let x = diff / FRAC_PI_2;
                (x, -(1.0 - x))
            }
            BottomRight => {
                let x = diff / FRAC_PI_2;
                (-x, (1.0 - x))
            }
            BottomLeft => {
                let z = diff / FRAC_PI_2;
                (-(1.0 - z), -z)
            }
        };

        dbg!(diff, piece, x, z);
        self.position.x += x * MOVEMENT_SPEED;
        self.position.z += z * MOVEMENT_SPEED;

        // let future_z = (self.position.z - MOVEMENT_SPEED / 2.0 - MODEL_RADIUS).floor();
        //
        // if self.no_x_obstacles(future_z, lvl) {
        //     self.position.z -= MOVEMENT_SPEED;
        // }
    }
    pub fn move_back(&mut self, lvl: &Res<Level>) {
        let future_z = (self.position.z + MOVEMENT_SPEED / 2.0 + MODEL_RADIUS).floor();

        if self.no_x_obstacles(future_z, lvl) {
            self.position.z += MOVEMENT_SPEED;
        }
    }
    pub fn move_left(&mut self, lvl: &Res<Level>) {
        let movement_speed = MOVEMENT_SPEED / 2.0;
        let future_x = (self.position.x - movement_speed - MODEL_RADIUS).floor();

        if self.no_z_obstacles(future_x, lvl) {
            self.position.x -= movement_speed;
        }
    }
    pub fn move_right(&mut self, lvl: &Res<Level>) {
        let future_x = (self.position.x + MOVEMENT_SPEED / 2.0 + MODEL_RADIUS).floor();

        if self.no_z_obstacles(future_x, lvl) {
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
                let y_gap = self.position.y - self.position.y.floor();
                if self.can_fall(lvl) {
                    self.position.go_down(GRAVITY_SPEED);
                } else if y_gap > 0.0 {
                    let gravity_speed = if y_gap > GRAVITY_SPEED { GRAVITY_SPEED } else { y_gap };
                    self.position.go_down(gravity_speed);
                }

                if !self.should_fall(lvl) {
                    self.movement_state = None;
                }
            }
            MovementState::Jumping(tick) => {
                self.position.go_up(GRAVITY_SPEED);

                if tick == 10 {
                    self.movement_state = None;
                } else {
                    self.movement_state = Some(MovementState::Jumping(tick + 1));
                }
            }
        }
    }

    pub fn move_model(&self, position: &mut Transform) {
        position.translation = vec3(self.position.x, self.position.y + 1.0, self.position.z);
    }
}

// obstacles checks
impl Player {
    fn can_jump(&self, lvl: &Res<Level>) -> bool {
        self.has_fundament(lvl) && !self.has_ceil(lvl)
    }

    fn should_fall(&self, lvl: &Res<Level>) -> bool {
        self.position.y - self.position.y.floor() != 0.0 || !self.has_fundament(lvl)
    }
    fn can_fall(&self, lvl: &Res<Level>) -> bool {
        let future_y = self.position.y.floor();

        self.no_y_obstacles(future_y, lvl)
    }

    fn has_fundament(&self, lvl: &Res<Level>) -> bool {
        let mut points = vec![
            Point::new(self.position.x.floor(), self.position.y.floor(), self.position.z.floor()),
        ];

        let x_gap = round_based(self.position.x - self.position.x.floor(), 1);
        if x_gap > MODEL_RADIUS {
            points.push(Point::new(self.position.x.round(), self.position.y.floor(), self.position.z.round()));
        } else if x_gap < MODEL_RADIUS {
            points.push(Point::new((self.position.x - MODEL_RADIUS).floor(), self.position.y.floor(), self.position.z.round()));
        }

        let z_gap = round_based(self.position.z - self.position.z.floor(), 1);
        if z_gap > MODEL_RADIUS {
            points.push(Point::new(self.position.x.round(), self.position.y.floor(), self.position.z.round()));
        } else if z_gap < MODEL_RADIUS {
            points.push(Point::new(self.position.x.round(), self.position.y.floor(), (self.position.z - MODEL_RADIUS).floor()));
        }

        !self.all_air(&points, lvl)
    }
    fn has_ceil(&self, lvl: &Res<Level>) -> bool {
        let future_position = Point::new(self.position.x.round(), (self.position.y + 3.0).floor(), self.position.z.round());
        let voxel_ceil = lvl.get_voxel_by_point(&future_position);

        voxel_ceil.is_some()
    }

    fn no_y_obstacles(&self, y: f32, lvl: &Res<Level>) -> bool {
        const FALLING_MODEL_RADIUS: f32 = MODEL_RADIUS * 0.9;
        let obstacles = [
            Point::new((self.position.x + FALLING_MODEL_RADIUS).floor(), y, (self.position.z + FALLING_MODEL_RADIUS).floor()),
            Point::new((self.position.x + FALLING_MODEL_RADIUS).floor(), y, (self.position.z - FALLING_MODEL_RADIUS).floor()),
            Point::new((self.position.x - FALLING_MODEL_RADIUS).floor(), y, (self.position.z + FALLING_MODEL_RADIUS).floor()),
            Point::new((self.position.x - FALLING_MODEL_RADIUS).floor(), y, (self.position.z - FALLING_MODEL_RADIUS).floor()),
        ];

        self.all_air(&obstacles, lvl)
    }
    fn no_x_obstacles(&self, z: f32, lvl: &Res<Level>) -> bool {
        let x_gap = round_based(self.position.x - self.position.x.floor(), 2);

        let obstacles = if x_gap == MODEL_RADIUS {
            vec![
                Point::new(self.position.x.floor(), self.position.y + 1.0, z),
                Point::new(self.position.x.floor(), self.position.y + 2.0, z),
            ]
        } else if x_gap > MODEL_RADIUS {
            vec![
                Point::new((self.position.x + MODEL_RADIUS).round(), self.position.y + 1.0, z),
                Point::new((self.position.x + MODEL_RADIUS).round(), self.position.y + 2.0, z),
                Point::new(self.position.x.floor(), self.position.y + 1.0, z),
                Point::new(self.position.x.floor(), self.position.y + 2.0, z),
            ]
        } else {
            vec![
                Point::new((self.position.x - MODEL_RADIUS).floor(), self.position.y + 1.0, z),
                Point::new((self.position.x - MODEL_RADIUS).floor(), self.position.y + 2.0, z),
                Point::new(self.position.x.floor(), self.position.y + 1.0, z),
                Point::new(self.position.x.floor(), self.position.y + 2.0, z),
            ]
        };

        self.all_air(&obstacles, lvl)
    }
    fn no_z_obstacles(&self, x: f32, lvl: &Res<Level>) -> bool {
        let z_gap = round_based(self.position.z - self.position.z.floor(), 2);

        let obstacles = if z_gap == MODEL_RADIUS {
            vec![
                Point::new(x, self.position.y + 1.0, self.position.z.floor()),
                Point::new(x, self.position.y + 2.0, self.position.z.floor()),
            ]
        } else if z_gap > MODEL_RADIUS {
            vec![
                Point::new(x, self.position.y + 1.0, (self.position.z + MODEL_RADIUS).round()),
                Point::new(x, self.position.y + 2.0, (self.position.z + MODEL_RADIUS).round()),
                Point::new(x, self.position.y + 1.0, self.position.z.floor()),
                Point::new(x, self.position.y + 2.0, self.position.z.floor()),
            ]
        } else {
            vec![
                Point::new(x, self.position.y + 1.0, (self.position.z - MODEL_RADIUS).floor()),
                Point::new(x, self.position.y + 2.0, (self.position.z - MODEL_RADIUS).floor()),
                Point::new(x, self.position.y + 1.0, self.position.z.floor()),
                Point::new(x, self.position.y + 2.0, self.position.z.floor()),
            ]
        };

        self.all_air(&obstacles, lvl)
    }

    fn all_air(&self, points: &[Point], lvl: &Res<Level>) -> bool {
        points.iter()
            .all(|p| lvl.get_voxel_by_point(p).is_none())
    }
}

