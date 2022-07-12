use bevy::math::vec3;
use bevy::math::Vec3;
use serde::{Deserialize, Serialize};

use crate::util::math::round_based;

// TODO remove and use Vec3 everywhere
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

    pub fn into_vec3(&self) -> Vec3 {
        vec3(self.x, self.y, self.z)
    }
    pub fn floor(&self) -> Point {
        Point::new(self.x.floor(), self.y.floor(), self.z.floor())
    }
}
