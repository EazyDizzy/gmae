use std::time::Instant;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::renderer::RenderDevice;

use crate::entity::voxel::Shape;
use crate::level::Level;
use crate::level::render::material::TEXTURE_SIZE;
use crate::level::render::mesh::merge_voxels;
use crate::level::render::shape::cube::create_cube_bundle_batch;
use crate::level::render::voxel_sequence::VoxelSequence;
use crate::Material;
use crate::system::light::{spawn_blue_light_source_inside, spawn_orange_light_source_inside};

pub mod material;
mod mesh;
mod voxel_sequence;
pub mod shape;

#[allow(clippy::needless_pass_by_value)]
pub fn init_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    level: Res<Level>,
    render_device: Res<RenderDevice>,
) {
    let limits = render_device.limits().max_texture_dimension_2d;
    // This is needed because of wgpu limitation. It can't render a texture which breaks the limits in some dimension
    let max_voxels_per_dimension = limits / TEXTURE_SIZE;
    dbg!(max_voxels_per_dimension);

    let merged_voxels = merge_voxels(&level.voxels, max_voxels_per_dimension);

    let start = Instant::now();

    for sequence in &merged_voxels {
        match sequence.shape() {
            Shape::Cube => {
                spawn_cube_sequence(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut images,
                    sequence,
                    &merged_voxels,
                );
            }
            Shape::TrianglePrism(..) => {
                // spawn_triangle_prism_sequence(
                //     &mut commands,
                //     &mut meshes,
                //     &mut materials,
                //     &mut images,
                //     sequence,
                //     properties,
                //     &merged_voxels,
                // );
            }
        }
    }

    println!("world initialization: {:?}", start.elapsed());
}

// fn spawn_triangle_prism_sequence(
//     commands: &mut Commands,
//     meshes: &mut ResMut<Assets<Mesh>>,
//     materials: &mut ResMut<Assets<StandardMaterial>>,
//     images: &mut ResMut<Assets<Image>>,
//     sequence: &VoxelSequence,
//     properties: &TrianglePrismProperties,
//     merged_voxels: &Vec<VoxelSequence>,
// ) {
//     let batch = create_triangle_bundle_batch(meshes, materials, images, sequence, properties, merged_voxels);
//
//     for bundle in batch {
//         commands.spawn_bundle(bundle);
//     }
// }

fn spawn_cube_sequence(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    sequence: &VoxelSequence,
    merged_voxels: &Vec<VoxelSequence>,
) {
    let batch = create_cube_bundle_batch(meshes, materials, images, sequence, merged_voxels);
    let mut light_spawned = false;

    for bundle in batch {
        let mut entity_commands = commands.spawn_bundle(bundle);

        if !light_spawned {
            light_spawned = spawn_light(&mut entity_commands, sequence.material());
        }
    }
}

fn spawn_light(entity_commands: &mut EntityCommands, material: Material) -> bool {
    if material == Material::OrangeLight {
        spawn_orange_light_source_inside(entity_commands);
        true
    } else if material == Material::BlueLight {
        spawn_blue_light_source_inside(entity_commands);
        true
    } else {
        false
    }
}
