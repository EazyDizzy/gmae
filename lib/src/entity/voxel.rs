use std::collections::HashMap;
use bevy::math::Vec3;

use crate::entity::WorldSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Material {
    Solid,
    // TODO creatures can walk through such blocks
    Passable,
    Water,

    OrangeLight,
    BlueLight,
    Unknown,
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
    pub position: Vec3,
    pub material: Material,
    pub shape: Shape,
}

impl Voxel {
    pub fn new(position: Vec3, material: Material, shape: Shape) -> Voxel {
        Voxel {
            position,
            material,
            shape,
        }
    }
}
