use std::collections::HashMap;

use bevy::prelude::*;

use crate::entity::voxel::Voxel;
use crate::Material;

#[derive(Debug)]
struct VoxelSequence<'a> {
    start: &'a Voxel,
    end: &'a Voxel,
}

impl<'a> VoxelSequence<'a> {
    fn height(&self) -> u32 {
        (self.end.position.y + 1.0 - self.start.position.y) as u32
    }
}

pub fn merge_voxels_in_meshes(voxels: &[Voxel], max_voxels_per_dimension: u32) -> Vec<(shape::Box, &Voxel)> {
    let grouped_voxels = group_voxels_by_coordinates(voxels);

    let mut meshes = vec![];

    for (_, plate) in grouped_voxels {
        let mut plane_sequences = vec![];

        for (y, row) in &plate {
            let row_sequences = merge_voxels_row(row.clone(), max_voxels_per_dimension);

            plane_sequences = stretch_sequences_by_y(row_sequences, plane_sequences, *y, max_voxels_per_dimension);
        }

        for sequence in plane_sequences {
            let shape = shape::Box {
                min_x: 0.0,
                max_x: sequence.end.position.x - sequence.start.position.x + 1.0,
                min_y: 0.0,
                max_y: sequence.end.position.y - sequence.start.position.y + 1.0,
                min_z: 0.0,
                max_z: sequence.end.position.z - sequence.start.position.z + 1.0,
            };

            meshes.push((shape, sequence.start));
        }
    }

    meshes
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
            s.end.position.y == y as f32 - 1.0
        }).collect();

    for sequence in row_sequences {
        let same_sequence = prev_row_sequences.iter_mut().find(|s| {
            s.start.position.x == sequence.start.position.x
                && s.end.position.x == sequence.end.position.x
                && s.start.material == sequence.start.material
                && should_merge(sequence.start.material)
        });

        if let Some(same) = same_sequence {
            if same.height() + sequence.height() < max_voxels_per_dimension {
                same.end = sequence.end;
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
            x_sequences.push(
                VoxelSequence {
                    start: start_voxel,
                    end: prev_voxel,
                });

            start_voxel = voxel;
        }

        prev_voxel = voxel;
    }
    x_sequences.push(
        VoxelSequence {
            start: start_voxel,
            end: prev_voxel,
        });

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