use serde::{Deserialize, Serialize};

use crate::util::math::round_based;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }

    pub fn sub_y(&mut self, y: f32) {
        self.y -= y;
        self.y = round_based(self.y, 2);
    }
    pub fn add_y(&mut self, y: f32) {
        self.y += y;
        self.y = round_based(self.y, 2);
    }
}
