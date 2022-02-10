use bevy::prelude::*;

use crate::entity::voxel::{Fastening, TrianglePrismProperties, WorldSide};
use crate::level::render::material::merge_materials;
use crate::level::render::mesh::get_or_create;
use crate::level::render::shape::{is_back_side_visible, is_bottom_side_visible, is_forward_side_visible, is_left_side_visible, is_right_side_visible, is_top_side_visible};
use crate::level::render::voxel_sequence::VoxelSequence;

const PI: f32 = std::f32::consts::PI;

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
    let material = merge_materials(
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
                    material: material.clone(),
                    transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height / 2.0, pos.z),
                    ..Default::default()
                });
            }

            let slope_mesh = get_or_create(meshes, width, height, true);
            bundles.push(PbrBundle {
                mesh: slope_mesh,
                material: material.clone(),
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
                    material: material.clone(),
                    transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height / 2.0, pos.z - 1.0),
                    ..Default::default()
                });
            }

            let slope_width = if [WorldSide::North, WorldSide::South].contains(&properties.facing) {
                width * 1.41
            } else {
                width
            };
            let slope_height = if [WorldSide::North, WorldSide::South].contains(&properties.facing) {
                height
            } else {
                height * 1.41
            };
            let slope_mesh = get_or_create(meshes, slope_width, slope_height, false);
            bundles.push(PbrBundle {
                mesh: slope_mesh,
                material: material.clone(),
                transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height / 2.0, pos.z - 0.5)
                    .with_rotation(properties.facing.generate_slope_rotation()),
                ..Default::default()
            });
        }
    }

    let second_square_mesh = get_or_create(
        meshes,
        width,
        height,
        [WorldSide::North, WorldSide::East].contains(&properties.facing),
    );
    let second_square_bundle = match properties.facing {
        WorldSide::North => {
            PbrBundle {
                mesh: second_square_mesh,
                material,
                transform: Transform::from_xyz(pos.x, pos.y + height / 2.0, pos.z - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                ..Default::default()
            }
        }
        WorldSide::South => {
            PbrBundle {
                mesh: second_square_mesh,
                material: material.clone(),
                transform: Transform::from_xyz(pos.x + width, pos.y + height / 2.0, pos.z - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                ..Default::default()
            }
        }
        WorldSide::East => {
            PbrBundle {
                mesh: second_square_mesh,
                material: material.clone(),
                transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height, pos.z - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
                ..Default::default()
            }
        }
        WorldSide::West => {
            PbrBundle {
                mesh: second_square_mesh,
                material,
                transform: Transform::from_xyz(pos.x + width / 2.0, pos.y, pos.z - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
                ..Default::default()
            }
        }
    };
    bundles.push(second_square_bundle);

    bundles
}