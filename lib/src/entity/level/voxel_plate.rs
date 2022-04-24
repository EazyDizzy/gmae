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
        let z = voxel.position.z as usize;
        self.internal.entry(z)
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
        // println!("Posiible z {:?}", self.internal.keys().into_iter().map(|k| *k).collect::<Vec<usize>>());
        // println!("search z {} {}", point.z, point.z as usize);
        self.internal.get(&(point.z as usize))
            .and_then(|row| {
                // println!("row found {} {}", point.z, point.z as usize);

                row.iter()
                    .find(|v| v.position.x == point.x)
            })
    }
}