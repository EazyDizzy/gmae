use bevy::prelude::*;

use crate::level::render::material::merge_materials;
use crate::level::render::mesh::get_or_create;
use crate::level::render::shape::{is_back_side_visible, is_bottom_side_visible, is_forward_side_visible, is_left_side_visible, is_right_side_visible, is_top_side_visible};
use crate::level::render::voxel_sequence::VoxelSequence;

const PI: f32 = std::f32::consts::PI;

pub fn create_cube_bundle_batch(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    sequence: &VoxelSequence,
    merged_voxels: &Vec<VoxelSequence>,
) -> Vec<PbrBundle> {
    let mut bundles = vec![];
    let pos = sequence.start_position();
    let width = sequence.x_width();
    let height = sequence.y_height();

    // TODO fix visibility near triangles
    let top_side_visible = is_top_side_visible(sequence, &merged_voxels);
    let bottom_side_visible = is_bottom_side_visible(sequence, &merged_voxels);
    let right_side_visible = is_right_side_visible(sequence, &merged_voxels);
    let left_side_visible = is_left_side_visible(sequence, &merged_voxels);
    let forward_side_visible = is_forward_side_visible(sequence, &merged_voxels);
    let back_side_visible = is_back_side_visible(sequence, &merged_voxels);

    // Quad shapes use transform translation coordinates as their center. That's why a bonus of size/2 is added
    if top_side_visible || bottom_side_visible {
        if top_side_visible {
            let material = merge_materials(
                &sequence.top_voxels(),
                materials,
                images,
                width as u32,
                height as u32,
            );
            let mesh = get_or_create(meshes, width, height, false);
            bundles.push(PbrBundle {
                mesh,
                material: material.clone(),
                transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height / 2.0, pos.z),
                ..Default::default()
            });
        }

        if bottom_side_visible {
            let material = merge_materials(
                sequence.bottom_voxels(),
                materials,
                images,
                width as u32,
                height as u32,
            );
            let mesh = get_or_create(meshes, width, height, true);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height / 2.0, pos.z - 1.0),
                ..Default::default()
            });
        }
    }

    if right_side_visible || left_side_visible {
        if right_side_visible {
            let material = merge_materials(
                &sequence.right_voxels(),
                materials,
                images,
                1,
                height as u32,
            );
            let mesh = get_or_create(meshes, 1.0, height, false);
            bundles.push(PbrBundle {
                mesh,
                material: material.clone(),
                transform: Transform::from_xyz(pos.x + width, pos.y + height / 2.0, pos.z - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                ..Default::default()
            });
        }

        if left_side_visible {
            let material = merge_materials(
                &sequence.left_voxels(),
                materials,
                images,
                1,
                height as u32,
            );
            let mesh = get_or_create(meshes, 1.0, height, true);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(pos.x, pos.y + height / 2.0, pos.z - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                ..Default::default()
            });
        }
    }

    if forward_side_visible || back_side_visible {
        if forward_side_visible {
            let material = merge_materials(
                &sequence.forward_voxels(),
                materials,
                images,
                width as u32,
                1,
            );
            let mesh = get_or_create(meshes, width, 1.0, true);
            bundles.push(PbrBundle {
                mesh,
                material: material.clone(),
                transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height, pos.z - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
                ..Default::default()
            });
        }

        if back_side_visible {
            let material = merge_materials(
                &sequence.backward_voxels(),
                materials,
                images,
                width as u32,
                1,
            );
            let mesh = get_or_create(meshes, width, 1.0, false);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(pos.x + width / 2.0, pos.y, pos.z - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
                ..Default::default()
            });
        }
    }

    bundles
}