use crate::entity::point::Point;

#[derive(Debug, PartialEq)]
pub enum VoxelMaterial {
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

#[derive(Debug)]
pub struct Voxel {
    pub position: Point,
    pub material: VoxelMaterial,
}