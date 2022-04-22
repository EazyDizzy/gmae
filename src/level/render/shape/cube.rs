use bevy::prelude::*;
use lib::util::game_settings::GameSettings;

use crate::level::render::material::merge_materials;
use crate::level::render::mesh::get_or_create;
use crate::level::render::named_materials::NamedMaterials;
use crate::level::render::shape::{is_back_side_visible, is_left_side_visible, is_right_side_visible, is_top_side_visible};
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
    let width = sequence.x_width();
    let height = sequence.y_height();

    // TODO fix visibility near triangles
    let top_side_visible = is_top_side_visible(sequence, merged_voxels);
    let bottom_side_visible = false;
    let right_side_visible = is_right_side_visible(sequence, merged_voxels);
    let left_side_visible = is_left_side_visible(sequence, merged_voxels);
    let forward_side_visible = false;
    let back_side_visible = is_back_side_visible(sequence, merged_voxels);

    // Quad shapes use transform translation coordinates as their center. That's why a bonus of size/2 is added
    if top_side_visible || bottom_side_visible {
        if top_side_visible {
            let material = merge_materials(
                &sequence.top_voxels(),
                named_materials,
                materials,
                images,
                width as u32,
                height as u32,
                settings,
            );
            let mesh = get_or_create(meshes, width, height, false);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(start_pos.x + width / 2.0, start_pos.y + height / 2.0, end_pos.z - 0.5),
                ..Default::default()
            });
        }

        if bottom_side_visible {
            let material = merge_materials(
                sequence.bottom_voxels(),
                named_materials,
                materials,
                images,
                width as u32,
                height as u32,
                settings,
            );
            let mesh = get_or_create(meshes, width, height, true);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(start_pos.x + width / 2.0, start_pos.y + height / 2.0, start_pos.z - 1.5),
                ..Default::default()
            });
        }
    }

    let z_height = sequence.z_height();
    if right_side_visible || left_side_visible {
        if right_side_visible {
            let material = merge_materials(
                &sequence.right_voxels(),
                named_materials,
                materials,
                images,
                z_height as u32,
                height as u32,
                settings,
            );
            let mesh = get_or_create(meshes, z_height, height, false);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(start_pos.x + width, start_pos.y + height / 2.0, end_pos.z - z_height / 2.0 - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                ..Default::default()
            });
        }

        if left_side_visible {
            let material = merge_materials(
                &sequence.left_voxels(),
                named_materials,
                materials,
                images,
                z_height as u32,
                height as u32,
                settings,
            );
            let mesh = get_or_create(meshes, z_height, height, true);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(start_pos.x, start_pos.y + height / 2.0, end_pos.z - z_height / 2.0 - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
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
                width as u32,
                z_height as u32,
                settings,
            );
            let mesh = get_or_create(meshes, width, z_height, true);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(start_pos.x + width / 2.0, start_pos.y + height, end_pos.z - z_height / 2.0 - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
                ..Default::default()
            });
        }

        if back_side_visible {
            let material = merge_materials(
                &sequence.backward_voxels(),
                named_materials,
                materials,
                images,
                width as u32,
                z_height as u32,
                settings,
            );
            let mesh = get_or_create(meshes, width, z_height, false);
            bundles.push(PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(start_pos.x + width / 2.0, start_pos.y, end_pos.z - z_height / 2.0 - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
                ..Default::default()
            });
        }
    }

    bundles
}