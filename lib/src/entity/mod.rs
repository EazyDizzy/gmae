use serde::{Deserialize, Serialize};

pub mod level;
pub mod point;
pub mod voxel;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum WorldSide {
    North,
    South,
    East,
    West,
}

impl WorldSide {
    fn from_property(value: &str) -> WorldSide {
        match value {
            "north" => WorldSide::North,
            "south" => WorldSide::South,
            "east" => WorldSide::East,
            "west" => WorldSide::West,
            v => panic!("Unknown world_side value {v}"),
        }
    }
}
