use std::cmp;
use std::collections::HashMap;

use bevy::prelude::*;

use crate::entity::voxel::Voxel;
use crate::VoxelMaterial;

#[derive(Debug)]
struct VoxelSequence<'a> {
    start: &'a Voxel,
    end: &'a Voxel,
}

pub fn merge_voxels_in_meshes(voxels: &Vec<Voxel>) -> Vec<(shape::Box, &Voxel)> {
    let (grouped_voxels, min_y, max_y) = group_voxels_by_coordinates(voxels);

    let mut meshes = vec![];

    for (_, plate) in grouped_voxels {
        let mut xy_sequences = vec![];

        for y in min_y..=max_y {
            let y_row = plate.get(y);
            if y_row.is_none() {
                continue;
            }
            let row = y_row.unwrap().clone();
            if row.len() == 0 {
                continue;
            }

            let x_sequences = merge_voxels_row(row);

            xy_sequences = stretch_sequences_by_y(x_sequences, xy_sequences, y);
        }

        for sequence in xy_sequences {
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

fn stretch_sequences_by_y<'a>(x_sequences: Vec<VoxelSequence<'a>>, mut xy_sequences: Vec<VoxelSequence<'a>>, y: usize) -> Vec<VoxelSequence<'a>> {
    let mut sequences_to_append = vec![];
    let mut prev_row_sequences: Vec<&mut VoxelSequence> = xy_sequences.iter_mut()
        .filter(|s: &&mut VoxelSequence| {
            s.end.position.y == y as f32 - 1.0
        }).collect();
    for sequence in x_sequences {
        let same_sequence = prev_row_sequences.iter_mut().find(|s| {
            s.start.position.x == sequence.start.position.x
                && s.end.position.x == sequence.end.position.x
                && s.start.material == sequence.start.material
                && should_merge(sequence.start.material)
        });

        if let Some(same) = same_sequence {
            same.end = sequence.end;
        } else {
            sequences_to_append.push(sequence);
        }
    }

    xy_sequences.append(&mut sequences_to_append);

    xy_sequences
}

fn merge_voxels_row(mut row: Vec<&Voxel>) -> Vec<VoxelSequence> {
    row.sort_by(|a, b| {
        a.position.x.partial_cmp(&b.position.x).unwrap()
    });

    let mut x_sequences = vec![];
    let mut start_voxel = row[0];
    let mut prev_voxel = row[0];

    for voxel in row {
        if voxel.id == prev_voxel.id {
            continue;
        }

        let stop_concatenation = voxel.position.x != prev_voxel.position.x + 1.0
            || voxel.material != prev_voxel.material
            || !should_merge(prev_voxel.material);
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

fn group_voxels_by_coordinates(voxels: &Vec<Voxel>) -> (HashMap<String, Vec<Vec<&Voxel>>>, usize, usize) {
    let mut stats = HashMap::new();
    let mut min_y = usize::MAX;
    let mut max_y = 0;

    for voxel in voxels {
        let z = voxel.position.z.round().to_string();
        if stats.get(&z).is_none() {
            stats.insert(z.clone(), vec![]);
        }

        let plane = stats.get_mut(&z).unwrap();
        let y = voxel.position.y.round() as usize;
        if plane.get(y).is_none() {
            for _ in plane.len()..=y {
                plane.push(vec![]);
            }
        }

        plane[y].push(voxel);
        max_y = cmp::max(y, max_y);
        min_y = cmp::min(y, min_y);
    }

    (stats, min_y, max_y)
}

fn should_merge(material: VoxelMaterial) -> bool {
    ![
        VoxelMaterial::BlueLight,
        VoxelMaterial::OrangeLight,
        VoxelMaterial::Glass
    ].contains(&material)
}