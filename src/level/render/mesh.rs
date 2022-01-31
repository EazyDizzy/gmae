use std::cmp;
use std::collections::HashMap;

use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::utils::Uuid;

use crate::entity::voxel::Voxel;
use crate::VoxelMaterial;

const MESH_UUID: Uuid = Uuid::from_bytes([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
const STANDARD_BLOCK_MESH_ID: HandleId = HandleId::Id(MESH_UUID, 1);
const GRASS_MESH_ID: HandleId = HandleId::Id(MESH_UUID, 2);

pub fn get_entity_mesh(material: VoxelMaterial, meshes: &ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    match material {
        VoxelMaterial::Grass => { meshes.get_handle(GRASS_MESH_ID) }
        _ => meshes.get_handle(STANDARD_BLOCK_MESH_ID)
    }
}

pub fn setup(mut meshes: ResMut<Assets<Mesh>>) {
    let _ = meshes.set(STANDARD_BLOCK_MESH_ID, Mesh::from(shape::Cube { size: 1.0 }));
    let _ = meshes.set(GRASS_MESH_ID, Mesh::from(shape::Cube { size: 1.0 }));
}

#[derive(Debug)]
struct VoxelSequence<'a> {
    start: &'a Voxel,
    end: &'a Voxel,
}

pub fn concatenate_voxels(voxels: &Vec<Voxel>) -> Vec<(shape::Box, &Voxel)> {
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

    let mut meshes = vec![];

    for (_, plate) in stats {
        let mut xy_sequences = vec![];

        for y in min_y..=max_y {
            if let Some(row) = plate.get(y) {
                if row.len() == 0 {
                    continue;
                }

                let mut row = row.clone();

                row.sort_by(|a, b| {
                    a.position.x.partial_cmp(&b.position.x).unwrap()
                });

                let mut x_sequences = vec![];
                let mut start_voxel = row[0];
                let mut prev_voxel = row[0];

                for voxel in row.iter() {
                    if voxel.id == prev_voxel.id {
                        continue;
                    }

                    let stop_concatenation = voxel.position.x != prev_voxel.position.x + 1.0
                        || voxel.material != prev_voxel.material
                        || !should_concatenate(prev_voxel.material);
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

                let mut sequences_to_append = vec![];
                if xy_sequences.is_empty() {
                    sequences_to_append = x_sequences;
                } else {
                    let mut prev_row_sequences: Vec<&mut VoxelSequence> = xy_sequences.iter_mut()
                        .filter(|s: &&mut VoxelSequence| {
                            s.end.position.y == y as f32 - 1.0
                        }).collect();
                    for sequence in x_sequences {
                        let same_sequence = prev_row_sequences.iter_mut().find(|s| {
                            s.start.position.x == sequence.start.position.x
                                && s.end.position.x == sequence.end.position.x
                                && should_concatenate(sequence.start.material)
                        });

                        if let Some(same) = same_sequence {
                            same.end = sequence.end;
                        } else {
                            sequences_to_append.push(sequence);
                        }
                    }
                }

                xy_sequences.append(&mut sequences_to_append);
            }
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

fn should_concatenate(material: VoxelMaterial) -> bool {
    ![
        VoxelMaterial::BlueLight,
        VoxelMaterial::OrangeLight,
        VoxelMaterial::Glass
    ].contains(&material)
}