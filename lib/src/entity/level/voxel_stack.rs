use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::entity::level::Point;
use crate::entity::level::voxel_plate::VoxelPlate;
use crate::entity::voxel::Voxel;

#[derive(Default, Serialize, Deserialize)]
pub struct VoxelStack {
    internal: HashMap<usize, VoxelPlate>,
}

impl VoxelStack {
    fn add_voxel(&mut self, voxel: Voxel) {
        let z = voxel.position.z.round() as usize;
        self.internal.entry(z)
            .or_insert_with(VoxelPlate::default)
            .add_voxel(voxel);
    }

    pub fn plates(&self) -> Vec<(usize, &VoxelPlate)> {
        let mut keys: Vec<usize> = self.internal.keys().into_iter().map(|k| *k).collect();
        keys.sort_by(|z1, z2| { z1.cmp(z2) });

        keys.into_iter()
            .map(|key| (key, &self.internal[&key]))
            .collect()
    }

    pub fn get_voxel_by_point(&self, point: &Point) -> Option<&Voxel> {
        self.internal.get(&(point.z as usize))
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