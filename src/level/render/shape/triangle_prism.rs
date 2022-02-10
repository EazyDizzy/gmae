use bevy::prelude::*;
use crate::entity::voxel::{Fastening, TrianglePrismProperties};

use crate::level::render::material::merge_materials;
use crate::level::render::mesh::get_or_create;
use crate::level::render::shape::{is_back_side_visible, is_bottom_side_visible, is_forward_side_visible, is_left_side_visible, is_right_side_visible, is_top_side_visible};
use crate::level::render::voxel_sequence::VoxelSequence;

pub fn create_triangle_bundle_batch(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    sequence: &VoxelSequence,
    properties: &TrianglePrismProperties,
    merged_voxels: &Vec<VoxelSequence>,
) -> Vec<PbrBundle> {
    let mut bundles = vec![];
    let pos = sequence.start_position();
    let width = sequence.x_width();
    let height = sequence.y_height();

    // Don't create material when not needed
    let top_bottom_material = merge_materials(
        sequence.material(),
        materials,
        images,
        width as u32,
        height as u32,
    );

    match properties.fastening {
        Fastening::Top => {
            if is_top_side_visible(sequence, &merged_voxels) {
                let mesh = get_or_create(meshes, width, height, false);
                bundles.push(PbrBundle {
                    mesh,
                    material: top_bottom_material.clone(),
                    transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height / 2.0, pos.z),
                    ..Default::default()
                });
            }

            let slope_mesh = get_or_create(meshes, 1.0, 1.0, true);
            bundles.push(PbrBundle {
                mesh: slope_mesh,
                material: top_bottom_material.clone(),
                transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height / 2.0, pos.z - 0.5)
                    .with_rotation(properties.facing.generate_slope_rotation().inverse()),
                ..Default::default()
            });
        }
        Fastening::Bottom => {
            if is_bottom_side_visible(sequence, &merged_voxels) {
                let mesh = get_or_create(meshes, width, height, false);
                bundles.push(PbrBundle {
                    mesh,
                    material: top_bottom_material.clone(),
                    transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height / 2.0, pos.z - 1.0),
                    ..Default::default()
                });
            }

            let slope_mesh = get_or_create(meshes, 1.0, 1.0, false);
            bundles.push(PbrBundle {
                mesh: slope_mesh,
                material: top_bottom_material.clone(),
                transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height / 2.0, pos.z - 0.5)
                    .with_rotation(properties.facing.generate_slope_rotation()),
                ..Default::default()
            });
        }
    }

    bundles
}