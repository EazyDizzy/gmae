#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: isize, y: isize, z: f32) -> Point {
        Point { x: x as f32, y: y as f32, z }
    }
}