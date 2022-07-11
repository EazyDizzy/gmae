use std::ops::RangeInclusive;

use lib::entity::point::Point;
use lib::entity::voxel::{Shape, Voxel};

use crate::Material;

#[derive(Debug)]
pub struct VoxelSequence<'a> {
    start: &'a Voxel,
    end: &'a Voxel,
}

impl<'a> VoxelSequence<'a> {
    pub fn new(voxels: Vec<&'a Voxel>) -> VoxelSequence<'a> {
        VoxelSequence {
            start: voxels.first().unwrap(),
            end: voxels.last().unwrap(),
        }
    }

    pub fn expand_y_end(&mut self, other: Self) {
        self.end = other.end;
    }

    pub fn expand_z_end(&mut self, other: Self) {
        self.end = other.end;
    }

    pub fn start_position(&self) -> &Point {
        &self.start.position
    }

    pub fn end_position(&self) -> &Point {
        &self.end.position
    }

    pub fn same_x_size(&self, other: &Self) -> bool {
        other.start.position.x == self.start.position.x
            && other.end.position.x == self.end.position.x
    }
    pub fn same_z_size(&self, other: &Self) -> bool {
        other.start.position.z == self.start.position.z
            && other.end.position.z == self.end.position.z
    }

    pub fn has_z_end_on(&self, z: f32) -> bool {
        let (.., end_z) = self.z_borders();

        end_z == z
    }


    pub fn has_y_end_on(&self, y: f32) -> bool {
        let (.., end_y) = self.y_borders();

        end_y == y
    }
    pub fn y_borders(&self) -> (f32, f32) {
        let start_y = self.start.position.y;
        let end_y = self.end.position.y;

        (start_y, end_y)
    }
    pub fn z_borders(&self) -> (f32, f32) {
        let start_z = self.start.position.z;
        let end_z = self.end.position.z;

        (start_z, end_z)
    }
}
