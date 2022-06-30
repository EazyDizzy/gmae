use std::collections::HashMap;

use crate::entity::point::Point;
use crate::entity::WorldSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Material {
    Unknown,

    Glass,
    Hay,
    Pumpkin,
    WhiteTerracotta,
    Water,
    // Light
    OrangeLight,
    BlueLight,
    // Stone
    Bedrock,
    Stone,
    Cobblestone,
    MossyCobblestone,
    MossyStoneBricks,
    CrackedStoneBricks,
    ChiseledStoneBricks,
    SmoothStone,
    StoneBricks,
    // Ground
    Grass,
    Dirt,
    DirtPath,
    Podzol,
    CoarseDirt,
    Farmland,
    // Wood + Leaves
    OakLeaves,
    OakLog,
    OakPlanks,
    StrippedOakLog,
    AcaciaLeaves,
    AcaciaLog,
    AcaciaPlanks,
    StrippedAcaciaLog,
    BirchLeaves,
    BirchLog,
    BirchPlanks,
    StrippedBirchLog,
    JungleLeaves,
    JungleLog,
    JunglePlanks,
    StrippedJungleLog,
    DarkOakLeaves,
    DarkOakLog,
    DarkOakPlanks,
    StrippedDarkOakLog,
    SpruceLeaves,
    SpruceLog,
    SprucePlanks,
    StrippedSpruceLog,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Shape {
    Cube,
    TrianglePrism(TrianglePrismProperties),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrianglePrismProperties {
    pub fastening: Fastening,
    pub facing: WorldSide,
}

impl TrianglePrismProperties {
    pub fn from_properties(properties: &HashMap<String, String>) -> TrianglePrismProperties {
        let fastening = Fastening::from_property(
            properties
                .get("half")
                .expect("`half` property does not exist"),
        );

        let facing = WorldSide::from_property(
            properties
                .get("facing")
                .expect("`facing` property does not exist"),
        );

        TrianglePrismProperties { fastening, facing }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Fastening {
    Top,
    Bottom,
}

impl Fastening {
    fn from_property(value: &str) -> Fastening {
        match value {
            "bottom" => Fastening::Bottom,
            "top" => Fastening::Top,
            v => panic!("Unknown fastening value {v}"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
