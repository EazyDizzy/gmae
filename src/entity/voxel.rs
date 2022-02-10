use std::collections::HashMap;
use crate::entity::point::Point;

#[derive(Debug, PartialEq, Copy, Clone)]
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
    WhiteTerracotta,
    Farmland,
    StrippedOakLog,
    Water,
    SmoothStone,
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
    pub fn from_properties(properties: &HashMap<String, String>) -> TrianglePrismProperties{
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
            facing
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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
