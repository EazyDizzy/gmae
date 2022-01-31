use std::time::Instant;

use bevy::prelude::*;

use crate::level::porter::read_level;
use crate::level::render::material::get_material;
use crate::level::render::mesh::{concatenate_voxels};
use crate::system::light::{spawn_blue_light_source, spawn_orange_light_source};
use crate::VoxelMaterial;

pub mod material;
pub mod mesh;

pub fn render_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    let map = read_level("debug");

    let now = Instant::now();
    let concatenated_voxels = concatenate_voxels(&map);
    let time = now.elapsed().as_millis();
    println!("concatenate_voxels time {}ms", time);
    println!("concatenations {}", concatenated_voxels.len());
    for (mesh_form, voxel) in concatenated_voxels {
        let pos = &voxel.position;
        let material = get_material(voxel.material, &materials);
        let mesh = meshes.add(mesh_form);

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

