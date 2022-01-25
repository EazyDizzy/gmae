use crate::entity::point::Point;

#[derive(Debug, PartialEq)]
pub enum VoxelMaterial {
    Unknown,
    Bedrock,
    Stone,
    Grass,
    Dirt,
    WoodenPlanks,
    Light,
}

#[derive(Debug)]
pub struct Voxel {
    pub position: Point,
    pub material: VoxelMaterial,
}