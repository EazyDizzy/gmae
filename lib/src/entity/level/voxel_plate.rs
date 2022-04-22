use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::entity::level::Voxel;
use crate::entity::point::Point;

#[derive(Default, Serialize, Deserialize)]
pub struct VoxelPlate {
    internal: HashMap<usize, Vec<Voxel>>,
}

impl VoxelPlate {
    pub fn add_voxel(&mut self, voxel: Voxel) {
        let y = voxel.position.y.round() as usize;
        self.internal.entry(y)
            .or_insert_with(Vec::new)
            .push(voxel);
    }

    pub fn rows(&self) -> Vec<(usize, &Vec<Voxel>)> {
        let mut keys: Vec<usize> = self.internal.keys().into_iter().map(|k| *k).collect();
        keys.sort_by(|z1, z2| { z1.cmp(z2) });

        keys.into_iter()
            .map(|key| (key, &self.internal[&key]))
            .collect()
    }

    pub fn get_voxel_by_point(&self, point: &Point) -> Option<&Voxel> {
        self.internal.get(&(point.y as usize))
            .and_then(|row|
                row.iter()
                    .find(|v| v.position.x == point.x)
            )
    }
}