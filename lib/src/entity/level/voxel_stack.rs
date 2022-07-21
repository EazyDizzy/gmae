use std::collections::HashMap;

use crate::entity::level::voxel_plate::VoxelPlate;
use crate::entity::voxel::Material;
use crate::entity::voxel::Voxel;
use bevy::math::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct VoxelStack {
    internal: HashMap<usize, VoxelPlate>,
}

impl VoxelStack {
    pub fn voxels_by_material(&self, materials: &[Material]) -> Vec<&Voxel> {
        let mut result = vec![];
        for (_, plate) in &self.internal {
            let mut lights = plate.voxels_by_material(materials);
            result.append(&mut lights);
        }
        result
    }

    pub fn width(&self) -> usize {
        let biggest_plate = self
            .internal
            .iter()
            .max_by(|(.., plate1), (.., plate2)| plate1.width().cmp(&plate2.width()));
        let (.., plate) = biggest_plate.expect("Failed to find any rows in VoxelPlate");
        plate.width()
    }

    fn add_voxel(&mut self, voxel: Voxel) {
        let y = voxel.position.y as usize;
        self.internal
            .entry(y)
            .or_insert_with(VoxelPlate::default)
            .add_voxel(voxel);
    }

    pub fn plates(&self) -> Vec<(usize, &VoxelPlate)> {
        let mut keys: Vec<usize> = self.internal.keys().into_iter().map(|k| *k).collect();
        keys.sort_by(|y1, y2| y1.cmp(y2));

        keys.into_iter()
            .map(|key| (key, &self.internal[&key]))
            .collect()
    }

    pub fn get_voxel_by_point(&self, point: &Vec3) -> Option<&Voxel> {
        self.internal
            .get(&(point.y as usize))
            .and_then(|plate| plate.get_voxel_by_point(point))
    }
}

impl From<Vec<Voxel>> for VoxelStack {
    fn from(voxels: Vec<Voxel>) -> Self {
        let mut stack = VoxelStack::default();

        for voxel in voxels {
            stack.add_voxel(voxel);
        }

        stack
    }
}
