#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point {
    pub fn new(x: isize, y: isize, z: isize) -> Point {
        Point { x, y, z }
    }
}