use std::ops::RangeInclusive;

use crate::entity::point::Point;
use crate::entity::voxel::Voxel;
use crate::Material;

#[derive(Debug)]
pub struct VoxelSequence<'a> {
    pub start: &'a Voxel,
    end: &'a Voxel,
}

impl<'a> VoxelSequence<'a> {
    pub fn new(start: &'a Voxel, end: &'a Voxel) -> VoxelSequence<'a> {
        VoxelSequence {
            start,
            end,
        }
    }

    pub fn start_position(&self) -> &Point {
        &self.start.position
    }

    pub fn same_x_size_and_material(&self, other: &Self) -> bool {
        other.start.position.x == self.start.position.x
            && other.end.position.x == self.end.position.x
            && other.start.material == self.start.material
    }

    pub fn expand_end(&mut self, other: &Self) {
        self.end = other.end
    }

    pub fn height(&self) -> f32 {
        self.start.position.z
    }
    pub fn material(&self) -> Material {
        self.start.material
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
    use crate::entity::point::Point;
    use crate::entity::voxel::Voxel;
    use crate::level::render::voxel_sequence::VoxelSequence;
    use crate::Material;

    #[test]
    fn one_block_y_borders() {
        let start = Voxel::new(0, Point::new(0, 0, 0.0), Material::Unknown);
        let seq = VoxelSequence::new(
            &start,
            &start,
        );

        assert_eq!(
            seq.y_borders(),
            (0.0, 0.0)
        );
    }
}