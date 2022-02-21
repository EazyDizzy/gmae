use serde::{Deserialize, Serialize};

use crate::entity::voxel::Voxel;

#[derive(Serialize, Deserialize)]
pub struct Level {
    pub voxels: Vec<Voxel>,
    day_part: DayPart,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum DayPart {
    Day,
    Night,
}

impl Level {
    pub fn new(voxels: Vec<Voxel>, day_part: DayPart) -> Level {
        Level {
            voxels,
            day_part,
        }
    }

    pub fn is_day(&self) -> bool {
        self.day_part == DayPart::Day
    }
}