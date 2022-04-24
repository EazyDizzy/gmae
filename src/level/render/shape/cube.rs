use bevy::prelude::*;
use lib::util::game_settings::GameSettings;

use crate::level::render::material::merge_materials;
use crate::level::render::mesh::get_or_create;
use crate::level::render::named_materials::NamedMaterials;
use crate::level::render::shape::{is_back_side_visible, is_bottom_side_visible, is_forward_side_visible, is_left_side_visible, is_right_side_visible, is_top_side_visible};
use crate::level::render::voxel_sequence::VoxelSequence;

const PI: f32 = std::f32::consts::PI;

pub fn create_cube_bundle_batch(
    meshes: &mut ResMut<Assets<Mesh>>,
    named_materials: &mut ResMut<NamedMaterials>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    sequence: &VoxelSequence,
    merged_voxels: &[VoxelSequence],
    settings: &Res<GameSettings>,
) -> Vec<PbrBundle> {
    let mut bundles = vec![];
    let start_pos = sequence.start_position();
    let end_pos = sequence.end_position();
    let x_width = sequence.x_width();
    let z_width = sequence.z_width();

    // TODO fix visibility near triangles
    let top_side_visible = is_top_side_visible(sequence, merged_voxels);
    let bottom_side_visible = is_bottom_side_visible(sequence, merged_voxels);
    let right_side_visible = is_right_side_visible(sequence, merged_voxels);
    let left_side_visible = is_left_side_visible(sequence, merged_voxels);
    let forward_side_visible = is_forward_side_visible(sequence, merged_voxels);
    let back_side_visible = is_back_side_visible(sequence, merged_voxels);
    let back_side_visible = true;

    // Quad shapes use transform translation coordinates as their center. That's why a bonus of size/2 is added
    if top_side_visible || bottom_side_visible {
        if top_side_visible {
            let material = merge_materials(
                &sequence.top_voxels(),
                named_materials,
                materials,
                images,
                x_width as u32,
                z_width as u32,
                settings,
            );
            let mesh = get_or_create(meshes, x_width, z_width, false);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(sequence.center_x(), end_pos.y, sequence.center_z())
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0)),
                ..Default::default()
            });
        }

        if bottom_side_visible {
            let material = merge_materials(
                sequence.bottom_voxels(),
                named_materials,
                materials,
                images,
                x_width as u32,
                z_width as u32,
                settings,
            );
            let mesh = get_or_create(meshes, x_width, z_width, true);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(sequence.center_x(), start_pos.y - 1.0, sequence.center_z())
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0)),
                ..Default::default()
            });
        }
    }

    let y_height = sequence.y_height();
    if right_side_visible || left_side_visible {
        if right_side_visible {
            let material = merge_materials(
                &sequence.right_voxels(),
                named_materials,
                materials,
                images,
                y_height as u32,
                z_width as u32,
                settings,
            );
            let mesh = get_or_create(meshes, y_height, z_width, false);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(start_pos.x + x_width, sequence.center_y(), sequence.center_z())
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, -PI / 2.0, PI / 2.0, 0.0)),
                ..Default::default()
            });
        }

        if left_side_visible {
            let material = merge_materials(
                &sequence.left_voxels(),
                named_materials,
                materials,
                images,
                y_height as u32,
                z_width as u32,
                settings,
            );
            let mesh = get_or_create(meshes, y_height, z_width, true);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(start_pos.x, sequence.center_y(), sequence.center_z())
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, -PI / 2.0, PI / 2.0, 0.0)),
                ..Default::default()
            });
        }
    }

    if forward_side_visible || back_side_visible {
        if forward_side_visible {
            let material = merge_materials(
                &sequence.forward_voxels(),
                named_materials,
                materials,
                images,
                x_width as u32,
                y_height as u32,
                settings,
            );
            let mesh = get_or_create(meshes, x_width, y_height, true);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(sequence.center_x(), sequence.center_y(), start_pos.z),
                ..Default::default()
            });
        }

        if back_side_visible {
            let material = merge_materials(
                &sequence.backward_voxels(),
                named_materials,
                materials,
                images,
                x_width as u32,
                y_height as u32,
                settings,
            );
            let mesh = get_or_create(meshes, x_width, y_height, false);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(sequence.center_x(), sequence.center_y(), start_pos.z + z_width),
                ..Default::default()
            });
        }
    }

    bundles
}