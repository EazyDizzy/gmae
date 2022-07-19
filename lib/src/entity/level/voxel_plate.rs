use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::entity::level::Voxel;
use crate::entity::point::Point;
use crate::entity::voxel::Material;

#[derive(Default, Serialize, Deserialize)]
pub struct VoxelPlate {
    internal: HashMap<usize, Vec<Voxel>>,
}

impl VoxelPlate {
    pub fn lights(&self) -> Vec<&Voxel> {
        self.voxels_by_material(&[Material::BlueLight, Material::OrangeLight])
    }

    pub fn width(&self) -> usize {
        let max_row = self
            .internal
            .iter()
            .max_by(|(.., row1), (.., row2)| row1.len().cmp(&row2.len()));
        let (.., row) = max_row.expect("Failed to find any rows in VoxelPlate");
        row.len()
    }

    pub fn add_voxel(&mut self, voxel: Voxel) {
        let z = voxel.position.z as usize;
        self.internal.entry(z).or_insert_with(Vec::new).push(voxel);
    }

    pub fn rows(&self) -> Vec<(usize, &Vec<Voxel>)> {
        let mut keys: Vec<usize> = self.internal.keys().into_iter().map(|k| *k).collect();
        keys.sort_by(|z1, z2| z1.cmp(z2));

        keys.into_iter()
            .map(|key| (key, &self.internal[&key]))
            .collect()
    }

    pub fn get_voxel_by_point(&self, point: &Point) -> Option<&Voxel> {
        self.internal
            .get(&(point.z as usize))
            .and_then(|row| row.iter().find(|v| v.position.x == point.x))
    }

    pub fn voxels_by_material(&self, materials: &[Material]) -> Vec<&Voxel> {
        let mut result = vec![];
        for (_, voxels) in &self.internal {
            let mut needed_voxels = voxels
                .iter()
                .filter(|v| materials.contains(&v.material))
                .collect();
            result.append(&mut needed_voxels);
        }

        result
    }
}
