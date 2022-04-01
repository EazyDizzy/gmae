use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::entity::point::Point;
use crate::entity::voxel::Voxel;

#[derive(Serialize, Deserialize)]
pub struct Level {
    day_part: DayPart,

    #[serde(skip_serializing, skip_deserializing)]
    grouped_voxels: HashMap<usize, HashMap<usize, Vec<Voxel>>>,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum DayPart {
    Day,
    Night,
}

impl Level {
    pub fn new(voxels: Vec<Voxel>, day_part: DayPart) -> Level {
        Level {
            day_part,
            grouped_voxels: group_voxels_by_coordinates(voxels),
        }
    }

    pub fn is_day(&self) -> bool {
        self.day_part == DayPart::Day
    }

    pub fn grouped_voxels(&self) -> &HashMap<usize, HashMap<usize, Vec<Voxel>>> {
        &self.grouped_voxels
    }
}

fn group_voxels_by_coordinates(voxels: Vec<Voxel>) -> HashMap<usize, HashMap<usize, Vec<Voxel>>> {
    let mut grouping = HashMap::new();

    for voxel in voxels {
        let z = voxel.position.z.round() as usize;
        let z_plane = grouping.entry(z).or_insert_with(HashMap::new);

        let y = voxel.position.y.round() as usize;
        let y_row = z_plane.entry(y).or_insert_with(Vec::new);

        y_row.push(voxel);
    }

    grouping
}