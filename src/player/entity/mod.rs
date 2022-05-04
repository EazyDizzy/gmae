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
            position: Point::new(9.5, 1.0, 3.0),
            movement_state: None,
        }
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    pub fn move_forward(&mut self, lvl: &Res<Level>, angle: f32) {
        let (x_modifier, z_modifier) = angle_to_forward_x_z_modifiers(angle);
        let future_x = self.position.x + x_modifier * MOVEMENT_SPEED;
        let future_z = self.position.z + z_modifier * MOVEMENT_SPEED;

        self.go_to(future_x, future_z, lvl);
    }
    pub fn move_back(&mut self, lvl: &Res<Level>, angle: f32) {
        let (x_modifier, z_modifier) = angle_to_forward_x_z_modifiers(angle);
        let future_x = self.position.x - x_modifier * MOVEMENT_SPEED;
        let future_z = self.position.z - z_modifier * MOVEMENT_SPEED;

        self.go_to(future_x, future_z, lvl);
    }
    pub fn move_left(&mut self, lvl: &Res<Level>, angle: f32) {
        let (x_modifier, z_modifier) = angle_to_left_x_z_modifiers(angle);
        let future_x = self.position.x + x_modifier * MOVEMENT_SPEED;
        let future_z = self.position.z + z_modifier * MOVEMENT_SPEED;

        self.go_to(future_x, future_z, lvl);
    }
    pub fn move_right(&mut self, lvl: &Res<Level>, angle: f32) {
        let (x_modifier, z_modifier) = angle_to_left_x_z_modifiers(angle);
        let future_x = self.position.x - x_modifier * MOVEMENT_SPEED;
        let future_z = self.position.z - z_modifier * MOVEMENT_SPEED;

        self.go_to(future_x, future_z, lvl);
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
                    self.position.sub_y(GRAVITY_SPEED);
                } else if y_gap > 0.0 {
                    let gravity_speed = if y_gap > GRAVITY_SPEED {
                        GRAVITY_SPEED
                    } else {
                        y_gap
                    };
                    self.position.sub_y(gravity_speed);
                }

                if !self.should_fall(lvl) {
                    self.movement_state = None;
                }
            }
            MovementState::Jumping(tick) => {
                self.position.add_y(GRAVITY_SPEED);

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

    fn go_to(&mut self, future_x: f32, future_z: f32, lvl: &Res<Level>) {
        if self.can_stay_on(future_x, future_z, lvl) {
            self.position.x = future_x;
            self.position.z = future_z;
        }
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
        !self.no_y_obstacles(self.position.y, lvl)
    }
    fn has_ceil(&self, lvl: &Res<Level>) -> bool {
        let future_position = Point::new(
            self.position.x.round(),
            (self.position.y + 3.0).floor(),
            self.position.z.round(),
        );
        let voxel_ceil = lvl.get_voxel_by_point(&future_position);

        voxel_ceil.is_some()
    }

    fn can_stay_on(&self, x: f32, z: f32, lvl: &Res<Level>) -> bool {
        let mut obstacles: Vec<Point> = get_touched_points(x, self.position.y, z);

        obstacles = obstacles
            .iter()
            .map(|p| Point::new(p.x, p.y + 1.0, p.z))
            .collect();
        obstacles.extend(
            obstacles
                .iter()
                .map(|p| Point::new(p.x, p.y + 1.0, p.z))
                .collect::<Vec<Point>>(),
        );

        lvl.points_are_empty(&obstacles)
    }

    fn no_y_obstacles(&self, y: f32, lvl: &Res<Level>) -> bool {
        let obstacles: Vec<Point> = get_touched_points(self.position.x, y, self.position.z);

        lvl.points_are_empty(&obstacles)
    }
}

fn angle_to_pie_piece(angle: f32) -> (f32, PiePiece) {
    if angle <= FRAC_PI_2 {
        (angle, BottomLeft)
    } else if angle <= PI {
        (angle - FRAC_PI_2, TopLeft)
    } else if angle <= PI + FRAC_PI_2 {
        (angle - PI, TopRight)
    } else {
        (angle - (PI + FRAC_PI_2), BottomRight)
    }
}

fn angle_to_forward_x_z_modifiers(angle: f32) -> (f32, f32) {
    let (diff, piece) = angle_to_pie_piece(angle);

    match piece {
        TopLeft => {
            let x = diff / FRAC_PI_2;
            (x, -(1.0 - x))
        }
        TopRight => {
            let z = diff / FRAC_PI_2;
            ((1.0 - z), z)
        }
        BottomRight => {
            let x = diff / FRAC_PI_2;
            (-x, (1.0 - x))
        }
        BottomLeft => {
            let z = diff / FRAC_PI_2;
            (-(1.0 - z), -z)
        }
    }
}

fn angle_to_left_x_z_modifiers(angle: f32) -> (f32, f32) {
    let (diff, piece) = angle_to_pie_piece(angle);

    match piece {
        TopLeft => {
            let z = diff / FRAC_PI_2;
            (-(1.0 - z), -z)
        }
        TopRight => {
            let x = diff / FRAC_PI_2;
            (x, -(1.0 - x))
        }
        BottomRight => {
            let z = diff / FRAC_PI_2;
            ((1.0 - z), z)
        }
        BottomLeft => {
            let x = diff / FRAC_PI_2;
            (-x, (1.0 - x))
        }
    }
}

fn get_touched_points(x: f32, y: f32, z: f32) -> Vec<Point> {
    let mut points: Vec<Point> = vec![Point::new(x.floor(), y, z), Point::new(x, y, z.floor())];

    let x_gap = round_based(x - x.floor(), 1);
    if x_gap > MODEL_RADIUS {
        points.push(Point::new((x + MODEL_RADIUS).floor(), y, z.floor()));
    } else if x_gap < MODEL_RADIUS {
        points.push(Point::new((x - MODEL_RADIUS).floor(), y, z.floor()));
    };

    let z_gap = round_based(z - z.floor(), 1);
    if z_gap > MODEL_RADIUS {
        points.push(Point::new(x.floor(), y, (z + MODEL_RADIUS).floor()));
    } else if z_gap < MODEL_RADIUS {
        points.push(Point::new(x.floor(), y, (z - MODEL_RADIUS).floor()));
    };

    if x_gap > MODEL_RADIUS && z_gap > MODEL_RADIUS {
        points.push(Point::new(
            (x + MODEL_RADIUS).floor(),
            y,
            (z + MODEL_RADIUS).floor(),
        ));
    } else if x_gap < MODEL_RADIUS && z_gap < MODEL_RADIUS {
        points.push(Point::new(
            (x - MODEL_RADIUS).floor(),
            y,
            (z - MODEL_RADIUS).floor(),
        ));
    } else if x_gap > MODEL_RADIUS && z_gap < MODEL_RADIUS {
        points.push(Point::new(
            (x + MODEL_RADIUS).floor(),
            y,
            (z - MODEL_RADIUS).floor(),
        ));
    } else if x_gap < MODEL_RADIUS && z_gap > MODEL_RADIUS {
        points.push(Point::new(
            (x - MODEL_RADIUS).floor(),
            y,
            (z + MODEL_RADIUS).floor(),
        ));
    }

    points
}
