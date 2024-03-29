use crate::entity::level::creature::Creature;
use crate::entity::level::voxel_stack::VoxelStack;
use crate::entity::voxel::Material;
use crate::entity::voxel::Voxel;
use bevy::math::Vec3;
use serde::{Deserialize, Serialize};

pub mod creature;
pub mod voxel_plate;
pub mod voxel_stack;

#[derive(Serialize, Deserialize)]
pub struct Level {
    pub name: String,
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
    pub fn new(
        name: String,
        voxels: Vec<Voxel>,
        day_part: DayPart,
        creatures: Vec<Creature>,
    ) -> Level {
        Level {
            name,
            day_part,
            voxel_stack: VoxelStack::from(voxels),
            creatures,
        }
    }

    pub fn width(&self) -> usize {
        self.voxel_stack.width()
    }

    pub fn lights(&self) -> Vec<&Voxel> {
        self.voxel_stack
            .voxels_by_material(&[Material::BlueLight, Material::OrangeLight])
    }

    pub fn is_day(&self) -> bool {
        self.day_part == DayPart::Day
    }

    pub fn voxel_stack(&self) -> &VoxelStack {
        &self.voxel_stack
    }

    pub fn get_voxel_by_point(&self, point: &Vec3) -> Option<&Voxel> {
        self.voxel_stack.get_voxel_by_point(point)
    }

    pub fn points_are_empty(&self, points: &[Vec3]) -> bool {
        points.iter().all(|p| self.get_voxel_by_point(p).is_none())
    }

    pub fn creatures(&self) -> &Vec<Creature> {
        &self.creatures
    }
}
