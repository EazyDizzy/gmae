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
}

#[derive(Debug, PartialEq, Clone)]
pub struct Voxel {
    pub id: usize,
    pub position: Point,
    pub material: Material,
}

impl Voxel {
    pub fn new(id: usize, position: Point, material: Material) -> Voxel {
        Voxel {
            id,
            position,
            material,
        }
    }
}