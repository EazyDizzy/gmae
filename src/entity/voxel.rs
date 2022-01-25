use crate::entity::point::Point;

#[derive(Debug)]
pub enum VoxelMaterial {
    Unknown,
    Bedrock,
    Stone,
    Grass,
    Dirt,
}

#[derive(Debug)]
pub struct Voxel {
    pub position: Point,
    pub material: VoxelMaterial,
}