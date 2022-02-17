use std::ops::RangeInclusive;

use lib::entity::point::Point;
use lib::entity::voxel::{Shape, Voxel};
use crate::Material;

#[derive(Debug)]
pub struct VoxelSequence<'a> {
    start: &'a Voxel,
    end: &'a Voxel,
    voxels: Vec<Vec<Vec<&'a Voxel>>>,
}

impl<'a> VoxelSequence<'a> {
    pub fn new(voxels: Vec<&'a Voxel>) -> VoxelSequence<'a> {
        VoxelSequence {
            start: voxels.first().unwrap(),
            end: voxels.last().unwrap(),
            voxels: vec![vec![voxels]],
        }
    }

    pub fn expand_z_end(&mut self, other: Self) {
        self.end = other.end;
        let only_plate = other.voxels[0].clone();
        self.voxels.push(only_plate);
    }

    pub fn expand_y_end(&mut self, other: Self) {
        self.end = other.end;
        let only_row = other.voxels[0][0].clone();
        self.voxels[0].push(only_row);
    }

    pub fn top_voxels(&self) -> Vec<Vec<&'a Voxel>> {
        let mut voxels = self.voxels.last().unwrap().clone();
        voxels.reverse();

        voxels
    }
    pub fn bottom_voxels(&self) -> &Vec<Vec<&'a Voxel>> {
        &self.voxels.first().unwrap()
    }

    pub fn right_voxels(&self) -> Vec<Vec<&'a Voxel>> {
        let mut voxels: Vec<Vec<&'a Voxel>> = vec![];

        for plate in &self.voxels {
            let mut plate_voxels: Vec<&'a Voxel> = plate.iter()
                .map(|row| *row.last().unwrap())
                .collect();
            plate_voxels.reverse();
            for (i, voxel) in plate_voxels.into_iter().enumerate() {
                if voxels.len() > i {
                    voxels[i].insert(0, voxel);
                } else {
                    voxels.push(vec![voxel]);
                }
            }
        }

        voxels
    }
    pub fn left_voxels(&self) -> Vec<Vec<&'a Voxel>> {
        let mut voxels: Vec<Vec<&'a Voxel>> = vec![];

        for plate in &self.voxels {
            let mut plate_voxels: Vec<&'a Voxel> = plate.iter()
                .map(|row| row[0])
                .collect();
            plate_voxels.reverse();
            for (i, voxel) in plate_voxels.into_iter().enumerate() {
                if voxels.len() > i {
                    voxels[i].insert(0, voxel);
                } else {
                    voxels.push(vec![voxel]);
                }
            }
        }
        voxels.reverse();

        voxels
    }

    pub fn forward_voxels(&self) -> Vec<Vec<&'a Voxel>> {
        let mut voxels = vec![];

        for plate in &self.voxels {
            voxels.push(plate.last().unwrap().clone())
        }
        voxels.reverse();

        voxels
    }
    pub fn backward_voxels(&self) -> Vec<Vec<&'a Voxel>> {
        let mut voxels = vec![];

        for plate in &self.voxels {
            voxels.push(plate.first().unwrap().clone())
        }
        voxels.reverse();

        voxels
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
    pub fn same_y_size(&self, other: &Self) -> bool {
        other.start.position.y == self.start.position.y
            && other.end.position.y == self.end.position.y
    }

    pub fn example_material(&self) -> Material {
        self.start.material
    }
    pub fn shape(&self) -> &Shape {
        &self.start.shape
    }

    pub fn is_not_transparent(&self) -> bool {
        self.start.material != Material::Glass
    }

    pub fn intersects_by_y(&self, other: &Self) -> bool {
        let (start_y, end_y) = other.y_borders();

        self.contains_y(start_y) || self.contains_y(end_y)
    }
    pub fn intersects_by_x(&self, other: &Self) -> bool {
        let (start_x, end_x) = other.x_borders();

        self.contains_x(start_x) || self.contains_x(end_x)
    }

    pub fn has_z_end_on(&self, z: f32) -> bool {
        let (.., end_z) = self.z_borders();

        end_z == z
    }
    pub fn z_height(&self) -> f32 {
        let (start_z, end_z) = self.z_borders();

        end_z - start_z + 1.0
    }

    pub fn has_x_start_on(&self, x: f32) -> bool {
        let (start_x, ..) = self.x_borders();

        start_x == x
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

    pub fn has_same_height(&self, other: &Self) -> bool {
        self.has_height(other.start.position.z)
    }
    pub fn has_height(&self, z: f32) -> bool {
        self.start.position.z == z
    }

    pub fn covered_coordinates(&self) -> Vec<(usize, usize)> {
        let mut coordinates = Vec::with_capacity((self.x_width() * self.y_height()) as usize);

        for y in self.covered_y() {
            for x in self.covered_x() {
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

    pub fn x_width(&self) -> f32 {
        let (start_x, end_x) = self.x_borders();
        end_x - start_x + 1.0
    }
    pub fn y_height(&self) -> f32 {
        let (start_y, end_y) = self.y_borders();
        end_y - start_y + 1.0
    }

    pub fn x_borders(&self) -> (f32, f32) {
        let start_x = self.start.position.x;
        let end_x = self.end.position.x;

        (start_x, end_x)
    }
    pub fn y_borders(&self) -> (f32, f32) {
        let start_y = self.start.position.y;
        let end_y = self.end.position.y;

        (start_y, end_y)
    }
    fn z_borders(&self) -> (f32, f32) {
        let start_z = self.start.position.z;
        let end_z = self.end.position.z;

        (start_z, end_z)
    }

    fn contains_y(&self, y: f32) -> bool {
        let (start_y, end_y) = self.y_borders();

        y >= start_y && y <= end_y
    }
    fn contains_x(&self, x: f32) -> bool {
        let (start_x, end_x) = self.x_borders();

        x >= start_x && x <= end_x
    }
}

#[cfg(test)]
mod tests {
    use lib::entity::point::Point;
    use lib::entity::voxel::Voxel;
    use lib::entity::level::render::voxel_sequence::VoxelSequence;
    use crate::Material;

// #[test]
    // fn one_block_y_borders() {
    //     let start = Voxel::new(Point::new(0, 0, 0.0), Material::Unknown);
    //     let seq = VoxelSequence::new(
    //         &start,
    //         &start,
    //     );
    //
    //     assert_eq!(
    //         seq.y_borders(),
    //         (0.0, 0.0)
    //     );
    // }
}