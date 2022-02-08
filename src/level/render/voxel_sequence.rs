use std::ops::RangeInclusive;

use bevy::prelude::*;

use crate::entity::voxel::Voxel;
use crate::Material;

#[derive(Debug)]
pub struct VoxelSequence<'a> {
    pub start: &'a Voxel,
    pub end: &'a Voxel,
}

impl<'a> VoxelSequence<'a> {
    pub fn height(&self) -> f32 {
        self.start.position.z
    }

    pub fn is_not_transparent(&self) -> bool {
        self.start.material != Material::Glass
    }

    pub fn intersects_by_y(&self, other: &Self) -> bool {
        let (start_y, end_y) = other.y_borders();

        self.contains_y(start_y) || self.contains_y(end_y)
    }
    pub fn contains_y(&self, y: f32) -> bool {
        let (start_y, end_y) = self.y_borders();

        y >= start_y && y <= end_y
    }
    pub fn intersects_by_x(&self, other: &Self) -> bool {
        let (start_x, end_x) = other.x_borders();

        self.contains_x(start_x) || self.contains_x(end_x)
    }
    pub fn contains_x(&self, x: f32) -> bool {
        let (start_x, end_x) = self.x_borders();

        x >= start_x && x <= end_x
    }

    pub fn has_x_start_on(&self, x: f32) -> bool {
        self.start.position.x == x
    }
    pub fn has_x_end_on(&self, x: f32) -> bool {
        let (.., end_x) = self.x_borders();

        end_x == x
    }
    pub fn has_y_start_on(&self, y: f32) -> bool {
        self.start.position.y == y
    }
    pub fn has_y_end_on(&self, y: f32) -> bool {
        let (.., end_y) = self.y_borders();

        end_y == y
    }

    pub fn has_height(&self, z: f32) -> bool {
        self.start.position.z == z
    }

    pub fn covered_coordinates(&self) -> Vec<(usize, usize)> {
        let (start_x, end_x) = self.x_borders();
        let (start_y, end_y) = self.y_borders();

        let mut coordinates = vec![];
        // TODO use covered_x/y
        for y in start_y as usize..end_y as usize {
            for x in start_x as usize..end_x as usize {
                coordinates.push((x, y));
            }
        }

        coordinates
    }
    pub fn covered_x(&self) -> RangeInclusive<usize> {
        let (start_x, end_x) = self.x_borders();

        start_x as usize..=end_x as usize
    }
    pub fn covered_y(&self) -> RangeInclusive<usize> {
        let (start_y, end_y) = self.y_borders();

        start_y as usize..=end_y as usize
    }

    pub fn sequence_height(&self) -> u32 {
        (self.end.position.y + 1.0 - self.start.position.y) as u32
    }

    pub fn get_box(&self) -> shape::Box {
        shape::Box {
            min_x: 0.0,
            max_x: self.x_width(),
            min_y: 0.0,
            max_y: self.y_height(),
            min_z: 0.0,
            max_z: self.end.position.z - self.start.position.z + 1.0,
        }
    }

    fn x_width(&self) -> f32 {
        self.end.position.x - self.start.position.x + 1.0
    }
    fn y_height(&self) -> f32 {
        self.end.position.y - self.start.position.y + 1.0
    }

    pub fn x_borders(&self) -> (f32, f32) {
        let start_x = self.start.position.x;
        let end_x = self.start.position.x + self.x_width();

        (start_x, end_x)
    }
    pub fn y_borders(&self) -> (f32, f32) {
        let start_y = self.start.position.y;
        let end_y = self.start.position.y + self.y_height();

        (start_y, end_y)
    }
}