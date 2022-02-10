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
    TrianglePrism,
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
