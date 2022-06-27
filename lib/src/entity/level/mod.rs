use serde::{Deserialize, Serialize};

use crate::entity::level::creature::Creature;
use crate::entity::level::voxel_stack::VoxelStack;
use crate::entity::point::Point;
use crate::entity::voxel::Voxel;

pub mod creature;
pub mod voxel_plate;
pub mod voxel_stack;

#[derive(Serialize, Deserialize)]
pub struct Level {
    day_part: DayPart,

    voxel_stack: VoxelStack,
    creatures: Vec<Creature>,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum DayPart {
    Day,
    Night,
}

impl Level {
    pub fn new(voxels: Vec<Voxel>, day_part: DayPart, creatures: Vec<Creature>) -> Level {
        Level {
            day_part,
            voxel_stack: VoxelStack::from(voxels),
            creatures,
        }
    }

    pub fn is_day(&self) -> bool {
        self.day_part == DayPart::Day
    }

    pub fn voxel_stack(&self) -> &VoxelStack {
        &self.voxel_stack
    }

    pub fn get_voxel_by_point(&self, point: &Point) -> Option<&Voxel> {
        self.voxel_stack.get_voxel_by_point(point)
    }

    pub fn points_are_empty(&self, points: &[Point]) -> bool {
        points.iter().all(|p| self.get_voxel_by_point(p).is_none())
    }

    pub fn creatures(&self) -> &Vec<Creature> {
        &self.creatures
    }
}
