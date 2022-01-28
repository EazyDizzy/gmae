use std::cmp;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use bevy::prelude::*;

use crate::entity::voxel::{Voxel, VoxelMaterial};
use crate::level::porter::read_level;
use crate::level::render::material::get_material;
use crate::level::render::mesh::get_entity_mesh;
use crate::system::light::{spawn_blue_light_source, spawn_orange_light_source};

pub mod material;
pub mod mesh;

pub fn render_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    let map = read_level("debug");

    let now = Instant::now();
    let concatenated_voxels = concatenate_voxels(&map);
    let time = now.elapsed().as_millis();
    println!("concatenate_voxels time {}ms", time);
    println!("concatenations {}", concatenated_voxels.len());
    for (mesh_form, voxel) in concatenated_voxels {
        let pos = &voxel.position;
        let material = get_material(voxel.material, &materials);
        let mesh = meshes.add(mesh_form);

        let _ = commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..Default::default()
        });
    }
}

#[derive(Debug)]
struct XSequence<'a> {
    start: &'a Voxel,
    end: &'a Voxel,
}

fn concatenate_voxels(voxels: &Vec<Voxel>) -> Vec<(Mesh, &Voxel)> {
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

    for (z, plate) in stats {
        let z = z.parse::<usize>().unwrap();

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

                    let is_end_of_sequence = voxel.position.x != prev_voxel.position.x + 1.0
                        || voxel.material != prev_voxel.material;
                    if is_end_of_sequence {
                        x_sequences.push(
                            XSequence {
                                start: start_voxel,
                                end: prev_voxel,
                            });

                        start_voxel = voxel;
                    }

                    prev_voxel = voxel;
                }
                x_sequences.push(
                    XSequence {
                        start: start_voxel,
                        end: prev_voxel,
                    });

                for sequence in x_sequences {
                    let shape = shape::Box {
                        min_x: 0.0,
                        max_x: sequence.end.position.x - sequence.start.position.x + 1.0,
                        min_y: 0.0,
                        max_y: sequence.end.position.y - sequence.start.position.y + 1.0,
                        min_z: 0.0,
                        max_z: sequence.end.position.z - sequence.start.position.z + 1.0,
                    };

                    meshes.push((Mesh::from(shape), sequence.start));
                }
            }
        }
    }

    meshes
}
