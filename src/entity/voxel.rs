use std::collections::HashMap;

use crate::entity::point::Point;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Material {
    Unknown,
    Bedrock,
    Stone,
    Grass,
    Dirt,
    WoodenPlanks,
    OrangeLight,
    BlueLight,
    DirtPath,
    Glass,
    Hay,
    Pumpkin,
    Cobblestone,
    MossyCobblestone,
    OakLeaves,
    OakLog,
    SpruceLog,
    StrippedSpruceLog,
    SpruceLeaves,
    WhiteTerracotta,
    Farmland,
    StrippedOakLog,
    StrippedDarkOakLog,
    Water,
    SmoothStone,
    Podzol,
    CoarseDirt,
    StoneBricks,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Cube,
    TrianglePrism(TrianglePrismProperties),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TrianglePrismProperties {
    pub fastening: Fastening,
    pub facing: WorldSide,
}

impl TrianglePrismProperties {
    pub fn from_properties(properties: &HashMap<String, String>) -> TrianglePrismProperties {
        let fastening = Fastening::from_property(
            properties.get("half")
                .expect("`half` property does not exist")
        );

        let facing = WorldSide::from_property(
            properties.get("facing")
                .expect("`facing` property does not exist")
        );

        TrianglePrismProperties {
            fastening,
            facing,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Fastening {
    Top,
    Bottom,
}

impl Fastening {
    fn from_property(value: &str) -> Fastening {
        match value {
            "bottom" => Fastening::Bottom,
            "top" => Fastening::Top,
            v => panic!("Unknown fastening value {v}")
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
            v => panic!("Unknown world_side value {v}")
        }
    }

    // pub fn generate_slope_rotation(&self) -> Quat {
    //     const PI: f32 = std::f32::consts::PI;
    //
    //     match self {
    //         WorldSide::North => {
    //             Quat::from_euler(EulerRot::XYZ, 0.0, PI / 4.0, 0.0)
    //         }
    //         WorldSide::South => {
    //             Quat::from_euler(EulerRot::XYZ, 0.0, -(PI / 4.0), 0.0)
    //         }
    //         WorldSide::East => {
    //             Quat::from_euler(EulerRot::XYZ, PI / 4.0, 0.0, 0.0)
    //         }
    //         WorldSide::West => {
    //             Quat::from_euler(EulerRot::XYZ, -(PI / 4.0), 0.0, 0.0)
    //         }
    //     }
    // }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Voxel {
    pub position: Point,
    pub material: Material,
    pub shape: Shape,
}

impl Voxel {
    pub fn new(position: Point, material: Material, shape: Shape) -> Voxel {
        Voxel {
            position,
            material,
            shape,
        }
    }
}
