use std::cmp;
use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use crate::entity::point::Point;
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

    let (concatenated_voxels, not_used_voxels) = concatenate_voxels(&map);
    for (mesh_form, voxel) in concatenated_voxels {
        let pos = &voxel.position;
        let material = get_material(voxel.material, &materials);
        let mesh = meshes.add(mesh_form);

        commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..Default::default()
        });
    }

    for voxel in not_used_voxels {
        let pos = &voxel.position;
        let material = get_material(voxel.material, &materials);
        let mesh = get_entity_mesh(voxel.material, &meshes);

        let mut entity_commands = commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..Default::default()
        });

        if voxel.material == VoxelMaterial::OrangeLight {
            spawn_orange_light_source(pos.x, pos.y, pos.z, &mut entity_commands);
        }
        if voxel.material == VoxelMaterial::BlueLight {
            spawn_blue_light_source(pos.x, pos.y, pos.z, &mut entity_commands);
        }
    }
}

fn concatenate_voxels(voxels: &Vec<Voxel>) -> (Vec<(Mesh, &Voxel)>, Vec<&Voxel>) {
    let mut stats = HashMap::new();

    for voxel in voxels {
        let z = voxel.position.z.round().to_string();
        if stats.get(&z).is_none() {
            stats.insert(z.clone(), vec![]);
        }

        stats.get_mut(&z).unwrap().push(voxel);
    }

    let mut meshes = vec![];
    let mut all_busy_voxels = vec![];

    for (_, row) in stats {
        let mut busy_voxels = vec![];

        for voxel in &row {
            let neighbours: Vec<&&Voxel> = row.iter().filter(|b| {
                if busy_voxels.contains(*b) {
                    return false;
                }
                if b.material != voxel.material {
                    return false;
                }

                let x_diff = (b.position.x as usize).abs_diff(voxel.position.x as usize);
                let y_diff = (b.position.y as usize).abs_diff(voxel.position.y as usize);

                x_diff <= 1 && y_diff <= 1
            }).collect();

            if neighbours.len() == 9 {
                for neighbour in &neighbours {
                    busy_voxels.push(**neighbour);
                }
                meshes.push((Mesh::from(
                    shape::Box::new(3.0, 3.0, 1.0)

                    //     shape::Box {
                    //     min_x: min_x as f32,
                    //     max_x: max_x as f32,
                    //     min_y: min_y as f32,
                    //     max_y: max_y as f32,
                    //     min_z: z,
                    //     max_z: z + 1.0,
                    // }
                ), *voxel));
            }
        }

        all_busy_voxels.append(&mut busy_voxels);
    }

    let not_used_voxels: Vec<&Voxel> = voxels.iter().filter(|v| !all_busy_voxels.contains(v)).collect();

    (meshes, not_used_voxels)
}
