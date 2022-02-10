use std::collections::HashMap;

use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::utils::Uuid;

use crate::entity::voxel::Voxel;
use crate::level::render::voxel_sequence::VoxelSequence;
use crate::Material;

pub fn get_or_create(meshes: &mut ResMut<Assets<Mesh>>, width: f32, height: f32, flip: bool) -> Handle<Mesh> {
    let handle_id = generate_mesh_handle_id(width, height, flip);

    if meshes.get(handle_id).is_none() {
        meshes.set(handle_id, Mesh::from(shape::Quad { size: Vec2::new(width, height), flip }))
    } else {
        meshes.get_handle(handle_id)
    }
}

pub fn merge_voxels(voxels: &[Voxel], max_voxels_per_dimension: u32) -> Vec<VoxelSequence> {
    let grouped_voxels = group_voxels_by_coordinates(voxels);

    let mut all_sequences = vec![];

    let mut z_keys: Vec<&usize> = grouped_voxels.keys().into_iter().collect();
    z_keys.sort_by(|z1, z2| { z1.cmp(z2) });

    for z in z_keys {
        let plate = &grouped_voxels[z];
        let mut y_keys: Vec<&usize> = plate.keys().into_iter().collect();
        y_keys.sort_by(|y1, y2| { y1.cmp(y2) });

        let mut plane_sequences = vec![];

        for y in y_keys {
            let row = plate[y].clone();
            let row_sequences = merge_voxels_row(row, max_voxels_per_dimension);

            plane_sequences = stretch_sequences_by_y(row_sequences, plane_sequences, *y, max_voxels_per_dimension);
        }

        all_sequences.extend(plane_sequences);
    }

    all_sequences
}

fn stretch_sequences_by_y<'a>(
    row_sequences: Vec<VoxelSequence<'a>>,
    mut plane_sequences: Vec<VoxelSequence<'a>>,
    y: usize,
    max_voxels_per_dimension: u32,
) -> Vec<VoxelSequence<'a>> {
    let mut sequences_to_append = vec![];
    let mut prev_row_sequences: Vec<&mut VoxelSequence> = plane_sequences.iter_mut()
        .filter(|s: &&mut VoxelSequence| {
            s.has_y_end_on(y as f32 - 1.0)
        }).collect();

    for sequence in row_sequences {
        let same_sequence = prev_row_sequences.iter_mut().find(|s| {
            s.same_x_size_and_material(&sequence)
                && should_merge(sequence.material())
        });

        if let Some(same) = same_sequence {
            if (same.y_height() as u32) + (sequence.y_height() as u32) < max_voxels_per_dimension {
                same.expand_end(&sequence);
            } else {
                sequences_to_append.push(sequence);
            }
        } else {
            sequences_to_append.push(sequence);
        }
    }

    plane_sequences.append(&mut sequences_to_append);

    plane_sequences
}

fn merge_voxels_row(mut row: Vec<&Voxel>, max_voxels_per_dimension: u32) -> Vec<VoxelSequence> {
    row.sort_by(|a, b| {
        a.position.x.partial_cmp(&b.position.x).unwrap()
    });

    let mut x_sequences = vec![];
    let mut start_voxel = row[0];
    let mut prev_voxel = row[0];

    for voxel in row.into_iter().skip(1) {
        let concatenation_width = (prev_voxel.position.x - start_voxel.position.x) as u32;
        let stop_concatenation = voxel.position.x != prev_voxel.position.x + 1.0
            || voxel.material != prev_voxel.material
            || !should_merge(prev_voxel.material)
            || concatenation_width + 1 == max_voxels_per_dimension;

        if stop_concatenation {
            x_sequences.push(VoxelSequence::new(start_voxel, prev_voxel));

            start_voxel = voxel;
        }

        prev_voxel = voxel;
    }
    x_sequences.push(VoxelSequence::new(start_voxel, prev_voxel));

    x_sequences
}

fn group_voxels_by_coordinates(voxels: &[Voxel]) -> HashMap<usize, HashMap<usize, Vec<&Voxel>>> {
    let mut grouping = HashMap::new();

    for voxel in voxels {
        let z = voxel.position.z.round() as usize;
        let z_plane = grouping.entry(z).or_insert_with(HashMap::new);

        let y = voxel.position.y.round() as usize;
        let y_row = z_plane.entry(y).or_insert_with(Vec::new);

        y_row.push(voxel);
    }

    grouping
}

fn should_merge(material: Material) -> bool {
    ![
        Material::BlueLight,
        Material::OrangeLight
    ].contains(&material)
}

fn generate_mesh_handle_id(width: f32, height: f32, flip: bool) -> HandleId {
    let id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, width as u8, height as u8, u8::from(flip)]);

    HandleId::Id(id, 1)
}