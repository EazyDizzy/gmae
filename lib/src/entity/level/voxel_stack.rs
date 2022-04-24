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
        let y = voxel.position.y as usize;
        self.internal.entry(y)
            .or_insert_with(VoxelPlate::default)
            .add_voxel(voxel);
    }

    pub fn plates(&self) -> Vec<(usize, &VoxelPlate)> {
        let mut keys: Vec<usize> = self.internal.keys().into_iter().map(|k| *k).collect();
        keys.sort_by(|y1, y2| { y1.cmp(y2) });

        keys.into_iter()
            .map(|key| (key, &self.internal[&key]))
            .collect()
    }

    pub fn get_voxel_by_point(&self, point: &Point) -> Option<&Voxel> {
        // println!("Posiible y {:?}", self.internal.keys().into_iter().map(|k| *k).collect::<Vec<usize>>());

        self.internal.get(&(point.y as usize))
            .and_then(|plate| {
                // println!("plate found {} {}", point.y, point.y as usize);
                plate.get_voxel_by_point(point)
            })
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