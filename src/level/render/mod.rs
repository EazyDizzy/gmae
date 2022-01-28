use bevy::prelude::*;

use crate::entity::voxel::VoxelMaterial;
use crate::level::porter::read_level;
use crate::level::render::material::get_material;
use crate::level::render::mesh::get_entity_mesh;
use crate::system::light::{spawn_blue_light_source, spawn_orange_light_source};

pub mod material;
pub mod mesh;

pub fn render_world(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    let map = read_level("lvl1");

    for voxel in map {
        let pos = voxel.position;
        let material = get_material(voxel.material, &materials);
        let mesh = get_entity_mesh(voxel.material, &meshes);

        let mut entity_commands = commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..Default::default()
        });

        if voxel.material == VoxelMaterial::OrangeLight {
            spawn_orange_light_source(pos.x, pos.y, pos.z, &mut entity_commands);
        }
        if voxel.material == VoxelMaterial::BlueLight {
            spawn_blue_light_source(pos.x, pos.y, pos.z, &mut entity_commands);
        }
    }
}
