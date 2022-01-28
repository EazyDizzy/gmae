use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::utils::Uuid;

use crate::VoxelMaterial;

const MESH_UUID: Uuid = Uuid::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
const STANDARD_BLOCK_MESH_ID: HandleId = HandleId::Id(MESH_UUID, 1);
const GRASS_MESH_ID: HandleId = HandleId::Id(MESH_UUID, 2);

pub fn get_entity_mesh(material: VoxelMaterial, meshes: &ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    match material {
        VoxelMaterial::Grass => {meshes.get_handle(GRASS_MESH_ID)}
        _ => meshes.get_handle(STANDARD_BLOCK_MESH_ID)
    }
}

pub fn setup(mut meshes: ResMut<Assets<Mesh>>) {
    let _ = meshes.set(STANDARD_BLOCK_MESH_ID, Mesh::from(shape::Cube { size: 1.0 }));
    let _ = meshes.set(GRASS_MESH_ID, Mesh::from(shape::Cube { size: 1.0 }));
}
