use bevy::prelude::*;

use crate::entity::voxel::VoxelMaterial;
use crate::level::read_level;
use crate::level::render::material::get_material;
use crate::system::light::{spawn_blue_light_source, spawn_orange_light_source};

pub mod material;

pub fn render_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    let map = read_level("debug");

    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    for voxel in map {
        let pos = voxel.position;
        let material = get_material(voxel.material, &materials);

        let mut entity_commands = commands.spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
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
